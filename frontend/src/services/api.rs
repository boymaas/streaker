use anyhow::Result;
use lazy_static::lazy_static;
use log;
use serde::Deserialize;
use serde_json::json;
use yew::callback::Callback;
use yew::format::Nothing;
use yew::format::{Json, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use url::Url;

use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug)]
pub enum ApiError {
    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Http Request Error")]
    RequestError,
}

lazy_static! {
    pub static ref END_POINT: Url =
        Url::parse("http://localhost:8080").expect("need a valid API endpoint");
}

// aggressove function to build api url
// will panic when it not resolves
fn build_api_url(path: &str) -> String {
    END_POINT
        .join(path)
        .expect("cannot form api url, check code")
        .to_string()
}

pub struct Api {
    fetcher: FetchService,
}

#[derive(Deserialize, Debug)]
pub struct JwtToken {
    pub token: String,
}

impl Api {
    pub fn new() -> Self {
        Self {
            fetcher: FetchService::new(),
        }
    }

    /// fetches token from API
    pub fn token_fetch(
        &mut self,
        callback: Callback<Result<JwtToken, ApiError>>,
    ) -> Result<FetchTask> {
        // build the requets, not that this builder requires body to be called
        // and the body type will be used in the fetch call
        // which will have to transform it into Result<String,Error>
        // this is why we need to use the Nothing format
        let request = Request::post(build_api_url("/api/v1/token/fetch"))
            .header("Content-Type", "application/json")
            .body(Nothing)
            .expect("failed to build request");

        // this handler will get the json returned from our
        // API. Parse it into the expected structure, and return
        // the required structure to the user
        let handler = move |response: Response<Text>| {
            if let (meta, Ok(data)) = response.into_parts() {
                log::info!("Response {:?}", data);
                if meta.status.is_success() {
                    let deserialized: Result<JwtToken, _> = serde_json::from_str(&data);
                    if let Ok(data) = deserialized {
                        callback.emit(Ok(data));
                    } else {
                        callback.emit(Err(ApiError::DeserializeError));
                    }
                } else {
                    match meta.status.as_u16() {
                        401 => callback.emit(Err(ApiError::Unauthorized)),
                        403 => callback.emit(Err(ApiError::Forbidden)),
                        404 => callback.emit(Err(ApiError::NotFound)),
                        500 => callback.emit(Err(ApiError::InternalServerError)),
                        _ => callback.emit(Err(ApiError::RequestError)),
                    }
                }
            } else {
                callback.emit(Err(ApiError::RequestError))
            }
        };

        self.fetcher.fetch(request, handler.into())
    }
}
