use chrono::Utc;
use futures::{FutureExt, StreamExt};
use log;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::ws::Message;

use sqlx::PgPool;

use streaker_common::streak_logic::StreakLogic;
use streaker_common::ws::{WsRequest, WsResponse};

use crate::jwt::{self, Claims, TokenData};
use crate::model::Member;
use crate::model::Scan;
use crate::model::ScanSession;

pub type WsChannel = mpsc::UnboundedSender<Result<Message, warp::Error>>;
pub type VisitorId = String;
pub type Sessions = Arc<RwLock<HashMap<Uuid, (Option<VisitorId>, WsChannel)>>>;

pub fn send_response(tx: &WsChannel, ws_response: &WsResponse) {
    tx.send(Ok(Message::text(
        serde_json::to_string(ws_response).unwrap(),
    )))
    .unwrap();
}

pub async fn handle(sessions: Sessions, pool: PgPool, token: String, socket: warp::ws::WebSocket) {
    // when we already have another tab open, we can send a message to the
    // previous open tab to close that one.

    let (ws_tx, mut ws_rx) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            log::error!("websocket send error: {}", e);
        }
    }));

    // lets look at our token, this also checks for the signature and if the token is expired. So
    // either its good, or we need to send a new token back, and reconnect.
    let token_data = jwt::decode_token(&token);
    if token_data.is_err() {
        // bad token, communicate this back to our frontend and reconnect
        let unauth_token = jwt::generate_token();
        send_response(&tx, &WsResponse::BadToken(unauth_token));
        return;
    }

    // here we have a valid token, authenticated or not
    let token_data = token_data.unwrap();

    // store the sender in our sessions
    // if we had an open connection, close that one
    if let Some((_, old_tx)) = sessions.write().await.insert(
        token_data.claims.suuid,
        (token_data.claims.visitorid.clone(), tx.clone()),
    ) {
        // the interface will respond to this action
        // by closing the websocket connection and logging out.
        send_response(&old_tx, &WsResponse::DoubleConnection);
    }

    // log all the keys we have in our sessions
    log::info!("{:?}", sessions.read().await.keys());

    // when we are here, we have either an authenticated
    // or unauthenticated session. When authenticated, we will
    // send over the state.
    send_response(&tx, &WsResponse::Connected);

    // acquire a connect, this now gives a hard error
    // TODO: is there another way to handle this
    let mut conn = pool.acquire().await.expect("Problem acquiring connection");

    // send over state when authenticated
    if let Claims {
        authenticated: true,
        visitorid: Some(visitorid),
        ..
    } = &token_data.claims
    {
        // member must exist, as it is created on authentication
        // process. If it does not exist, we have a structural problem
        //
        // visitorid is also always defined when we are authenticated
        if let Ok(member) = Member::fetch(&mut conn, &visitorid).await {
            // notice the clone here, as we are sending, and into consumes
            // the value
            send_response(&tx, &WsResponse::MemberState(member.clone().into()));

            // find our current scansession
            if let Ok(scan_session) = ScanSession::current(&mut conn, &visitorid).await {
                if let Ok(scan_session_state) = scan_session.scan_session_state(&mut conn).await {
                    send_response(&tx, &WsResponse::ScanSessionState(scan_session_state));
                }
            }

            // now lets build our StreakerState
            // for this we need our last registered scan. And some fields
            // of our member
            if let Ok(last_scan) = Scan::last_scan(&mut conn, visitorid).await {
                let streak_logic = StreakLogic::new(
                    member.streak_current,
                    member.streak_bucket,
                    last_scan.map(|ls| ls.tstamp),
                );
                let streak_state = streak_logic.evaluate(Utc::now());
                send_response(&tx, &WsResponse::StreakState(streak_state));
            }
        } else {
            log::error!("member must exist on authenticated connection!")
        };
    }

    log::info!("Connection established token_data={:?}", token_data);
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("websocket error(suuid={}) {}", token_data.claims.suuid, e);
                break;
            }
        };

        // TODO: catch this
        match msg.to_str() {
            Ok(msg) => {
                if let Ok(request) = serde_json::from_str::<WsRequest>(msg) {
                    // do something with message
                    handle_request(&sessions, &token_data, request).await;
                } else {
                    log::error!("Problem deserializing WebRequest")
                };
            }
            Err(e) => {
                // we get a non string message on a disconnect
                // for soem reason
                log::warn!("No string message");
            }
        }
    }
    // handle disconnect
    handle_disconnect(&sessions, &token_data).await;
}

async fn handle_disconnect(sessions: &Sessions, token_data: &TokenData<Claims>) {
    log::info!("disconneting session: {:?}", token_data);

    // Stream closed up, so remove from the user list
    sessions.write().await.remove(&token_data.claims.suuid);
}

async fn handle_request(sessions: &Sessions, token_data: &TokenData<Claims>, request: WsRequest) {
    match request {}
}
