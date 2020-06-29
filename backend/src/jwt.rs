use chrono;
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
    suuid: Uuid,
    authenticated: bool,
    company: String,
    concept: String,
}

lazy_static! {
    static ref SECRET: String =
        dotenv::var("STREAKER_SECRET").expect("STREAKER_SECRET needs to be set");
}

pub fn generate_token() -> String {
    // TODO: make the company and concept enviroment
    // varables
    let my_claims = Claims {
        exp: chrono::Utc::now().timestamp() as u64 + 24 * 60 * 60,
        suuid: Uuid::new_v4(),
        authenticated: false,
        company: "OPES Unite".to_owned(),
        concept: "Streaker".to_owned(),
    };

    // my_claims is a struct that implements Serialize
    // This will create a JWT using HS256 as algorithm
    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .unwrap()
}

fn fetch() {}