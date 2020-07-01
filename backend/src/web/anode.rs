// Now link the source of the attribution
// to the websocket channel, and generate
// a new token and send it over the websocket
use crate::jwt;
use crate::web::ws::{send_response, Sessions};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use warp;
use warp::reply::Json;

use streaker_common::ws::WsResponse;

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
) -> Result<Json, warp::reject::Rejection> {
    log::info!("{:?}", attr);
    if let Some(ws_channel) = ws_sessions.read().await.get(&attr.claim.suuid) {
        // now we have the channel so we can generate a new authenticated token
        // and send the update over the channel.
        // make sure the token has the same suuid, as this is tied to the websocket.
        let auth_token = jwt::generate_authenticated_token(attr.claim.suuid, attr.claim.visitorid);
        send_response(ws_channel, &WsResponse::Attribution(auth_token));
        Ok(warp::reply::json(&json!({"success": true})))
    } else {
        Err(warp::reject::not_found())
    }
}
