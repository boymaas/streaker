use anyhow::Error;
use lazy_static::lazy_static;
use log;
use serde_json::json;
use yew::callback::Callback;
use yew::format::Nothing;
use yew::format::{Json, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use url::Url;

// TODO: use dotenv

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

impl Api {
    pub fn new() -> Self {
        Self {
            fetcher: FetchService::new(),
        }
    }

    /// fetches token from API
    pub fn token_fetch(&mut self, callback: Callback<Text>) -> Result<FetchTask, Error> {
        // build the requets, not that this builder requires body to be called
        // and the body type will be used in the fetch call
        // which will have to transform it into Result<String,Error>
        // this is why we need to use the Nothing format
        let request = Request::post(build_api_url("/api/v1/token/fetch"))
            .header("Content-Type", "application/json")
            .body(Nothing)
            .expect("failed to build request");

        // this handler will get the json returned from our
        // API. Parse it into the expected structure.
        let handler = move |response: Response<Text>| {
            log::info!("Response {:?}", response);
        };

        self.fetcher.fetch(request, handler.into())
    }
}
