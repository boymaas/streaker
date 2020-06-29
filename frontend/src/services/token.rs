use jsonwebtoken::dangerous_unsafe_decode;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::RwLock;
use uuid::Uuid;
use yew::services::storage::{Area, StorageService};

// This should match our backend
// TODO: maybe create a shared include
#[derive(Debug, Deserialize)]
struct Claims {
    exp: u64,
    suuid: Uuid,
    authenticated: bool,
    company: String,
    concept: String,
}

const TOKEN_KEY: &str = "streaker.token";

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(token) = storage.restore(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(t) = token.clone() {
        storage.store(TOKEN_KEY, Ok(t));
    } else {
        storage.remove(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write().unwrap();
    *token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read().unwrap();
    token_lock.clone()
}

/// Check if current user is authenticated.
pub fn have_token() -> bool {
    get_token().is_some()
}

/// The idea here, is to just decode the jwt
/// token to see if this is an authenticated
/// token. The reasoning behind it is simple,
/// if its not signed, any request towards the
/// backend will return an error, and will log
/// the user out.
pub fn is_authenticated() -> bool {
    if let Some(token) = get_token() {
        if let Ok(parsed) = dangerous_unsafe_decode::<Claims>(&token) {
            parsed.claims.authenticated
        } else {
            false
        }
    } else {
        false
    }
}
