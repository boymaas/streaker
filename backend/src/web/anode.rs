// Now link the source of the attribution
// to the websocket channel, and generate
// a new token and send it over the websocket
use crate::jwt;
use crate::web::ws::{send_response, Sessions};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use warp;
use warp::reply::Json;

use crate::model::member;
use crate::model::scan_session::ScanSession;

use streaker_common::ws::{MemberState, WsResponse};

mod custom_date_parser {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};
    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // %e could be %d
        Utc.datetime_from_str(&s, "%a %b %e %H:%M:%S GMT %Y")
            .map_err(serde::de::Error::custom)
    }
}

mod source_parser {
    use super::{Source, SourceAction};
    use serde::{self, Deserialize, Deserializer};
    use uuid::Uuid;

    pub fn parse<'de, D>(deserializer: D) -> Result<Source, D::Error>
    where
        D: Deserializer<'de>,
    {
        let source = String::deserialize(deserializer)?;
        if let [action, suuid, ..] = source.split(":").collect::<Vec<&str>>().as_slice() {
            Ok(Source {
                suuid: Uuid::parse_str(suuid).map_err(serde::de::Error::custom)?,
                action: match *action {
                    "login" => SourceAction::Login,
                    "scan" => SourceAction::Scan,
                    _ => unreachable!(),
                },
            })
        } else {
            Err("problem parsing source").map_err(serde::de::Error::custom)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Claim {
    action: String,
    #[serde(deserialize_with = "source_parser::parse")]
    source: Source,
    #[serde(with = "custom_date_parser")]
    exp: DateTime<Utc>,
    visitorid: String,
}

#[derive(Debug)]
pub struct Source {
    pub action: SourceAction,
    pub suuid: Uuid,
}

#[derive(Debug)]
pub enum SourceAction {
    Login,
    Scan,
}

impl std::convert::Into<MemberState> for member::Member {
    fn into(self) -> MemberState {
        MemberState {
            visitorid: self.visitorid,
            bucket: self.bucket,
            streak_total: self.streak_total,
            streak_bucket: self.streak_bucket,
            balance: self.balance,
            email: self.email,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Attribution {
    access_node_name: String,
    claim: Claim,
}

// We receive an attribution from an access node, this can be
// either from a scan request of a login request. We dispatch
// to the appropiate handler.
pub async fn attribution(
    attr: Attribution,
    ws_sessions: Sessions,
    pool: PgPool,
) -> Result<Json, warp::reject::Rejection> {
    match attr.claim.source.action {
        SourceAction::Login => attribution_login(attr, ws_sessions, pool).await,
        SourceAction::Scan => attribution_scan(attr, ws_sessions, pool).await,
    }
}

// Handle the scan attribution, looking up the scansession for the
// logged in member.
async fn attribution_scan_inner(
    attr: Attribution,
    ws_sessions: Sessions,
    pool: PgPool,
) -> Result<Json> {
    // lets get the visitorid from this ws_connection, this visitorid
    // has been set on a websocket connection with an authenticated token.
    if let Some((Some(visitorid), ws_channel)) =
        ws_sessions.read().await.get(&attr.claim.source.suuid)
    {
        // link them together &attr.claim.visitorid
        let scan_session = ScanSession::current(&pool, visitorid).await?;

        scan_session
            .register_scan(&pool, &attr.access_node_name)
            .await?;

        let scan_session_state = scan_session.scan_session_state(&pool).await?;

        send_response(
            ws_channel,
            &WsResponse::ScanSessionState(scan_session_state),
        );
        Ok(warp::reply::json(&json!({"success": true})))
    } else {
        Err(anyhow::anyhow!("Session with visitorID not present!"))
    }
}

async fn attribution_scan(
    attr: Attribution,
    ws_sessions: Sessions,
    pool: PgPool,
) -> Result<Json, warp::reject::Rejection> {
    log::info!("SCAN: {:?}", attr);

    // TODO: the pattern here is warp filters need to return
    // a warp reply or reject, but errors do not map to the reject.
    // We need to find a pattern for this.
    //
    // The introcution of the inner is just to make the map_err happen
    // on one spot.
    attribution_scan_inner(attr, ws_sessions, pool)
        .await
        .map_err(|_| warp::reject::not_found())
}

// when we receive an attribution, and we have a session
// which matches the suuid of the attribution. We can authenticate
// that session with the correct visitor id
async fn attribution_login(
    attr: Attribution,
    ws_sessions: Sessions,
    pool: PgPool,
) -> Result<Json, warp::reject::Rejection> {
    log::info!("LOGIN: {:?}", attr);
    if let Some((_, ws_channel)) = ws_sessions.read().await.get(&attr.claim.source.suuid) {
        // now we have the channel so we can generate a new authenticated token
        // and send the update over the channel.
        // make sure the token has the same suuid, as this is tied to the websocket.
        let auth_token =
            jwt::generate_authenticated_token(&attr.claim.source.suuid, &attr.claim.visitorid);
        send_response(ws_channel, &WsResponse::Attribution(auth_token));

        // now also send out member state
        // fetch member from database, and build the member state
        if let Ok(member) = member::Member::fetch(&pool, &attr.claim.visitorid).await {
            // we have a member in the database, so lets send it over
            send_response(ws_channel, &WsResponse::MemberState(member.into()));
        } else {
            // we do not have a member in the database lets create one
            if let Ok(member) = member::Member::add(&pool, &attr.claim.visitorid).await {
                send_response(ws_channel, &WsResponse::MemberState(member.into()));
            } else {
                // we could not create this member, we need to communicate this back
                // to the UI
                send_response(
                    ws_channel,
                    &WsResponse::Error("Could not create MemberState".into()),
                );
            };
        };

        Ok(warp::reply::json(&json!({"success": true})))
    } else {
        Err(warp::reject::not_found())
    }
}
