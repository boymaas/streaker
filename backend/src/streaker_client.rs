use std::borrow::Cow;
use std::sync::Once;

use bytes::Bytes;
use chrono::{DateTime, Utc};
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
use crate::jwt::{decode_token, Claims, DecodedToken};
use crate::testdb::prepare_database;
use crate::web::anode;
use crate::web::ws;
use crate::web::StreakerApp;

#[derive(Debug, Deserialize)]
struct GetTokenResponse {
    token: String,
}

pub struct StreakerClient {
    pub streaker_app: StreakerApp,
    pub token_raw: Option<String>,
    pub token: Option<DecodedToken>,
    pub ws_client: Option<WsClient>,
    pub member_state: Option<MemberState>,
    pub scan_session_state: Option<ScanSessionState>,
    pub streak_state: Option<StreakState>,
    pub timefn: fn() -> DateTime<Utc>,
}

// https://stackoverflow.com/a/52934680
impl StreakerClient {
    pub fn new(streaker_app: StreakerApp) -> Self {
        Self {
            streaker_app,
            token: None,
            token_raw: None,
            ws_client: None,

            member_state: None,
            scan_session_state: None,
            streak_state: None,

            timefn: Utc::now,
        }
    }

    pub fn set_time(&mut self, timefn: fn() -> DateTime<Utc>) {
        self.timefn = timefn;
    }

    // connects like the web client would do. First fetch
    // a token if it does not exist. When we have the token
    // we connect to the websocket.
    pub async fn connect(&mut self) {
        let res = warp::test::request()
            .path("/api/v1/token/fetch")
            .method("POST")
            .header("content-type", "application/json")
            .reply(&self.streaker_app.routes(self.timefn))
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

    pub async fn ws_connect(&mut self) {
        // https://github.com/seanmonstar/warp/blob/master/tests/ws.rs
        let ws_path = format!("/ws/{}", self.token_raw.clone().unwrap());

        let mut client = warp::test::ws()
            .path(&ws_path)
            .handshake(self.streaker_app.route_ws(self.timefn))
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

    pub async fn post_attribution_login(&mut self, anode: &str, visitorid: &str) {
        // get the claims from our current token
        let claims = self.get_token_claims();
        let body = format!(
            r#"
        {{
           "access_node_name":"{}",
           "claim":{{
              "action":"checkin",
              "source":"{}:{}",
              "exp":"Fri Jul 10 10:58:34 GMT 2020",
              "visitorid":"{}"
           }}
        }}
        "#,
            anode, "login", claims.suuid, visitorid
        );

        let res = warp::test::request()
            .path("/api/v1/anode/attribution")
            .method("POST")
            .header("content-length", body.len())
            .header("content-type", "application/json")
            .body(body)
            .reply(&self.streaker_app.routes(self.timefn))
            .await;

        assert_eq!(res.status(), 200);

        self.ws_recv_attribution().await;
        self.ws_recv_member_state().await;

        // now we have a new token lets update our token and reconnect.
        self.ws_connect().await;

        self.ws_recv_member_state().await;
        self.ws_recv_scan_session_state().await;
        self.ws_recv_streak_state().await;
    }

    pub async fn post_attribution_scan(&mut self, anode: &str, visitorid: &str) {
        // get the claims from our current token
        let claims = self.get_token_claims();
        let body = format!(
            r#"
        {{
           "access_node_name":"{}",
           "claim":{{
              "action":"checkin",
              "source":"{}:{}",
              "exp":"Fri Jul 10 10:58:34 GMT 2020",
              "visitorid":"{}"
           }}
        }}
        "#,
            anode, "scan", claims.suuid, visitorid
        );

        let res = warp::test::request()
            .path("/api/v1/anode/attribution")
            .method("POST")
            .header("content-length", body.len())
            .header("content-type", "application/json")
            .body(body)
            .reply(&self.streaker_app.routes(self.timefn))
            .await;

        assert_eq!(res.status(), 200);

        self.ws_recv_member_state().await;
        self.ws_recv_scan_session_state().await;
        self.ws_recv_streak_state().await;
    }

    fn get_token_claims(&self) -> &Claims {
        self.token.as_ref().map(|t| &t.claims).unwrap()
    }

    async fn ws_recv_attribution(&mut self) {
        if let WsResponse::Attribution(token) = self.dump_ws_response().await {
            let decoded = decode_token(&token).unwrap();
            self.token_raw = Some(token.clone());
            self.token = Some(decoded);
        }
    }

    async fn ws_recv_member_state(&mut self) {
        if let WsResponse::MemberState(member_state) = self.dump_ws_response().await {
            self.member_state = Some(member_state);
        };
    }

    async fn ws_recv_scan_session_state(&mut self) {
        if let WsResponse::ScanSessionState(scan_session_state) = self.dump_ws_response().await {
            self.scan_session_state = Some(scan_session_state);
        };
    }

    async fn ws_recv_streak_state(&mut self) {
        if let WsResponse::StreakState(streak_state) = self.dump_ws_response().await {
            self.streak_state = Some(streak_state);
        };
    }

    async fn dump_ws_response(&mut self) -> WsResponse {
        let client = self.ws_client.as_mut().unwrap();
        let msg = client.recv().await.expect("recv");

        let response: WsResponse =
            serde_json::from_str::<WsResponse>(msg.to_str().unwrap()).unwrap();

        println!("{:#?}", response);

        response
    }
}
