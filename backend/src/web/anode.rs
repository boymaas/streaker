// Now link the source of the attribution
// to the websocket channel, and generate
// a new token and send it over the websocket
use anyhow::Result;
use chrono::{DateTime, Utc};
#[cfg(debug_assertions)]
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;
use warp;
use warp::reply::Json;

use streaker_common::streak_logic::StreakLogic;
use streaker_common::ws::{MemberState, WsResponse};

use crate::jwt;
use crate::model::member::Member;
use crate::model::scan::Scan;
use crate::model::scan_session::ScanSession;
use crate::web::ws::{send_response, Sessions};

mod login;
mod scan;

use crate::web::anode::login::attribution_login;
use crate::web::anode::scan::attribution_scan;

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
                    // In development mode we can do a scantest
                    // to test without going throught the production environment
                    // this gives us a source action in the format
                    // of scantest@( anode_label ):( uuid )
                    #[cfg(debug_assertions)]
                    _ => {
                        log::info!("action {}", action);
                        let scantest_r = regex::Regex::new(r#"^scantest@(.*?)$"#).unwrap();
                        if scantest_r.is_match(action) {
                            let anode_label = &scantest_r.captures(action).unwrap()[1];
                            log::info!("action {}", action);
                            log::info!("anode_label {}", anode_label);
                            SourceAction::ScanTest(anode_label.into())
                        } else {
                            unreachable!()
                        }
                    }
                    #[cfg(not(debug_assertions))]
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

type AccessNodeLabel = String;

#[derive(Debug)]
pub enum SourceAction {
    Login,
    Scan,
    #[cfg(debug_assertions)]
    ScanTest(AccessNodeLabel),
}

impl std::convert::Into<MemberState> for Member {
    fn into(self) -> MemberState {
        MemberState {
            visitorid: self.visitorid,
            streak_current: self.streak_current,
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
    mut attr: Attribution,
    ws_sessions: Sessions,
    pool: PgPool,
    time: DateTime<Utc>,
) -> Result<Json, warp::reject::Rejection> {
    match &attr.claim.source.action {
        SourceAction::Login => attribution_login(attr, ws_sessions, pool, &time).await,
        SourceAction::Scan => attribution_scan(attr, ws_sessions, pool, &time).await,
        #[cfg(debug_assertions)]
        SourceAction::ScanTest(anode) => {
            // here we overload the access_node_name witht the name
            // of the scantest so we can test scanning on a development
            // build.
            attr.access_node_name = anode.into();
            attribution_scan(attr, ws_sessions, pool, &time).await
        }
    }
}

#[derive(Debug)]
struct AttributionError(String);
impl warp::reject::Reject for AttributionError {}

// helper to reject with a message in a map_err context
pub fn reject<T: std::fmt::Debug>(msg: &str) -> Box<dyn FnOnce(T) -> warp::reject::Rejection> {
    let msg = msg.to_owned();
    Box::new(move |e: T| warp::reject::custom(AttributionError(format!("{}: {:?}", msg, e))))
}
