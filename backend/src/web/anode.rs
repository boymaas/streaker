// Now link the source of the attribution
// to the websocket channel, and generate
// a new token and send it over the websocket
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use warp;

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
    source: Uuid,
    #[serde(with = "custom_date_parser")]
    exp: DateTime<Utc>,
    visitorid: String,
}

#[derive(Deserialize, Debug)]
pub struct Attribution {
    access_node_name: String,
    claim: Claim,
}

pub fn attribution(attr: Attribution) -> warp::reply::Json {
    log::info!("{:?}", attr);
    warp::reply::json(&json!({"success": true}))
}
