use crate::jwt;
use serde_json::json;
use warp::reply::Json;

pub fn fetch() -> Json {
    // let json = json!({ "token": jwt::generate_token() }).to_string();
    warp::reply::json(&json!({ "token": jwt::generate_token() }))
}
