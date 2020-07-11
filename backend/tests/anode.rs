use bytes::Bytes;
use dotenv;
use pretty_env_logger;
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;
use std::borrow::Cow;
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::test::WsClient;
use warp::{http::Method, Filter, Rejection, Reply};

use tokio::sync::{mpsc, RwLock};

// since this is not a lib we need to mod
// modules as in the main
use streaker::jwt::{decode_token, DecodedToken};
use streaker::testdb::prepare_database;
use streaker::web::anode;
use streaker::web::ws;
use streaker::web::StreakerApp;

struct StreakerClient {
    streaker_app: StreakerApp,
    token_raw: Option<String>,
    token: Option<DecodedToken>,
    ws_client: Option<WsClient>,
}

impl StreakerClient {
    fn new(streaker_app: StreakerApp) -> Self {
        Self {
            streaker_app,
            token: None,
            token_raw: None,
            ws_client: None,
        }
    }

    // connects like the web client would do. First fetch
    // a token if it does not exist. When we have the token
    // we connect to the websocket.
    async fn connect(&mut self) {
        let res = warp::test::request()
            .path("/api/v1/token/fetch")
            .method("POST")
            .header("content-type", "application/json")
            .reply(&self.streaker_app.routes())
            .await;

        assert_eq!(res.status(), 200);

        // parse json response
        //
        let body = String::from_utf8_lossy(res.body());
        let response = serde_json::from_str::<GetTokenResponse>(&body).unwrap();

        let decoded = decode_token(&response.token).unwrap();
        //
        // Ensure our token is not authenticated, and visitorid
        // has nog been defined.
        assert_eq!(decoded.claims.authenticated, false);
        assert_eq!(decoded.claims.visitorid, None);

        self.token = Some(decoded);
        self.token_raw = Some(response.token.clone());

        // TODO: store the token, etc
    }

    async fn ws_connect(&mut self) {
        // https://github.com/seanmonstar/warp/blob/master/tests/ws.rs
        let ws_path = format!("/ws/{}", self.token_raw.clone().unwrap());

        let mut client = warp::test::ws()
            .path(&ws_path)
            .handshake(self.streaker_app.route_ws())
            .await
            .expect("could not do a handshake");

        println!(
            "Sessions: {:?}",
            self.streaker_app.sessions.read().await.keys()
        );

        // Now we expect to receive the Connected
        // message
        let msg = client.recv().await.expect("recv");
        assert_eq!(msg.to_str().unwrap(), "\"Connected\"");

        self.ws_client = Some(client);
    }
}

async fn prepare_test_app() -> StreakerApp {
    dotenv::dotenv().ok();

    // build and migrate database
    let pool = prepare_database().await;

    StreakerApp::new(pool)
}

#[derive(Debug, Deserialize)]
struct GetTokenResponse {
    token: String,
}

#[tokio::test]
async fn test_streaker_client() {
    let (app) = prepare_test_app().await;

    let mut client = StreakerClient::new(app);

    client.connect().await;
    client.ws_connect().await;
}

#[tokio::test]
async fn test_get_token() {
    let app = prepare_test_app().await;

    let res = warp::test::request()
        .path("/api/v1/token/fetch")
        .method("POST")
        .header("content-type", "application/json")
        .reply(&app.routes())
        .await;

    assert_eq!(res.status(), 200);

    // NOTE: res.body comes back as Bytes, need to
    // convert tot string
    let body = String::from_utf8_lossy(res.body());
    // println!("{:?}", body);

    let parsed: GetTokenResponse = serde_json::from_str(&body).unwrap();

    println!("{:?}", parsed);

    let decoded = decode_token(&parsed.token).unwrap();
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
