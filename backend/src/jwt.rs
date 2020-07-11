use chrono;
use dotenv::dotenv;
pub use jsonwebtoken::TokenData;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use anyhow::Result;

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub suuid: Uuid,
    pub authenticated: bool,
    pub visitorid: Option<String>,
    pub company: String,
    pub concept: String,
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
        visitorid: None,
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

pub fn generate_authenticated_token(suuid: &Uuid, visitorid: &String) -> String {
    // TODO: make the company and concept enviroment
    // varables
    let my_claims = Claims {
        exp: chrono::Utc::now().timestamp() as u64 + 24 * 60 * 60,
        suuid: Uuid::new_v4(),
        authenticated: true,
        visitorid: Some(visitorid.into()),
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

pub type DecodedToken = TokenData<Claims>;

pub fn decode_token(token: &str) -> Result<DecodedToken, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}
