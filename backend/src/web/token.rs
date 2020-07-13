use crate::jwt;
use chrono::{DateTime, Utc};
use serde_json::json;
use warp::reply::Json;

pub fn fetch(time: DateTime<Utc>) -> Json {
    // let json = json!({ "token": jwt::generate_token() }).to_string();
    warp::reply::json(&json!({ "token": jwt::generate_token() }))
}
