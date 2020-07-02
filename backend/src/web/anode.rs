// Now link the source of the attribution
// to the websocket channel, and generate
// a new token and send it over the websocket
use crate::jwt;
use crate::web::ws::{send_response, Sessions};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use warp;
use warp::reply::Json;

use crate::model::member;

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

#[derive(Deserialize, Debug)]
pub struct Claim {
    action: String,
    #[serde(rename = "source")]
    suuid: Uuid,
    #[serde(with = "custom_date_parser")]
    exp: DateTime<Utc>,
    visitorid: String,
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

// when we receive an attribution, and we have a session
// which matches the suuid of the attribution. We can authenticate
// that session with the correct visitor id
pub async fn attribution(
    attr: Attribution,
    ws_sessions: Sessions,
    pool: PgPool,
) -> Result<Json, warp::reject::Rejection> {
    log::info!("{:?}", attr);
    if let Some(ws_channel) = ws_sessions.read().await.get(&attr.claim.suuid) {
        // now we have the channel so we can generate a new authenticated token
        // and send the update over the channel.
        // make sure the token has the same suuid, as this is tied to the websocket.
        let auth_token =
            jwt::generate_authenticated_token(&attr.claim.suuid, &attr.claim.visitorid);
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
