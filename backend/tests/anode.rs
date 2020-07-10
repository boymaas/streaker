use dotenv;
use pretty_env_logger;
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{http::Method, Filter, Reply};

use tokio::sync::{mpsc, RwLock};

// since this is not a lib we need to mod
// modules as in the main
use streaker::jwt;
use streaker::testdb::prepare_database;
use streaker::web::anode;
use streaker::web::ws;
use streaker::web::StreakerApp;

async fn prepare_test_app() -> (StreakerApp, BoxedFilter<(impl Reply,)>) {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // build and migrate database
    let pool = prepare_database().await;

    // instantiate the app, and get the routes
    // to test against.
    let app = StreakerApp::new(pool);
    let routes = app.routes();

    (app, routes)
}

#[derive(Debug, Deserialize)]
struct GetTokenResponse {
    token: String,
}

#[tokio::test]
async fn test_get_token() {
    let (app, routes) = prepare_test_app().await;

    let res = warp::test::request()
        .path("/api/v1/token/fetch")
        .method("POST")
        .header("content-type", "application/json")
        .reply(&routes)
        .await;

    assert_eq!(res.status(), 200);

    println!("{:?}", res.body());
    let body = String::from_utf8_lossy(res.body());
    println!("{:?}", body);
    let parsed: GetTokenResponse = serde_json::from_str(&body).unwrap();
    println!("{:?}", parsed);

    let decoded = jwt::decode_token(&parsed.token).unwrap();
    println!("{:?}", decoded);

    // Ensure our token is not authenticated, and visitorid
    // has nog been defined.
    assert_eq!(decoded.claims.authenticated, false);
    assert_eq!(decoded.claims.visitorid, None);
    // assert_eq!(decoded.claim.exp, None );
}

#[tokio::test]
async fn test_login() {
    dotenv::dotenv().ok();

    // build and migrate database
    let pool = prepare_database().await;

    // instantiate the app, and get the routes
    // to test against.
    let app = StreakerApp::new(pool);
    let app_routes = app.routes();

    let uuid = Uuid::parse_str("be42b325-c990-43fa-a1e4-384aef09df1d").unwrap();
    let (tx, rx) = mpsc::unbounded_channel();
    app.sessions.write().await.insert(uuid, (None, tx));

    //
    let body = r#"
        {
           "access_node_name":"opesdentist",
           "claim":{
              "action":"checkin",
              "source":"login:be42b325-c990-43fa-a1e4-384aef09df1d",
              "exp":"Fri Jul 10 10:58:34 GMT 2020",
              "visitorid":"IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP"
           }
        }
        "#;

    let res = warp::test::request()
        .path("/api/v1/anode/attribution")
        .method("POST")
        .header("content-length", body.len())
        .header("content-type", "application/json")
        .body(body)
        .reply(&app_routes)
        .await;

    println!("{:?}", res);
    //
    // let res = warp::test::request().method("POST");
}
