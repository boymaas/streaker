use std::borrow::Cow;
use std::sync::Once;

use bytes::Bytes;
use dotenv;
use pretty_env_logger;
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::test::WsClient;
use warp::{http::Method, Filter, Rejection, Reply};

use tokio::sync::{mpsc, RwLock};

use streaker_common::ws::{MemberState, ScanSessionState, StreakState, WsResponse};

// since this is not a lib we need to mod
// modules as in the main
use streaker::jwt::{decode_token, Claims, DecodedToken};
use streaker::testdb::prepare_database;
use streaker::web::anode;
use streaker::web::ws;
use streaker::web::StreakerApp;

use streaker::streaker_client::StreakerClient;

static INIT: Once = Once::new();
async fn prepare_test_app() -> StreakerApp {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        pretty_env_logger::init();
    });

    // build and migrate database
    let pool = prepare_database().await;

    StreakerApp::new(pool)
}

#[tokio::test]
async fn test_streaker_client() {
    let app = prepare_test_app().await;

    let mut client = StreakerClient::new(app);

    client.connect().await;
    client.ws_connect().await;

    client
        .post_attribution_login("opesdentist", "IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP")
        .await;

    client
        .post_attribution_scan("opesdentist", "IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP")
        .await;
}
