use futures::{FutureExt, StreamExt};
use log;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};
use warp::Filter;

use streaker_common::ws::{WsRequest, WsResponse};

static NEXT_SESSION_ID: AtomicUsize = AtomicUsize::new(1);

pub type Sessions =
    Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

pub async fn handle(sessions: Sessions, socket: warp::ws::WebSocket) {
    let session_id = NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed);

    let (ws_tx, mut ws_rx) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // store the sender in our sessions
    sessions.write().await.insert(session_id, tx.clone());

    log::info!("{:?}", sessions.read().await.keys());

    // send something on connection
    tx.send(Ok(Message::text(
        serde_json::to_string(&WsResponse::Connected).unwrap(),
    )))
    .unwrap();

    log::info!("Connection established");
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(suid={}) {}", session_id, e);
                break;
            }
        };

        // TODO: catch this
        match msg.to_str() {
            Ok(msg) => {
                if let Ok(request) = serde_json::from_str::<WsRequest>(msg) {
                    // do something with message
                    handle_request(&sessions, session_id, request).await;
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
    handle_disconnect(&sessions, session_id).await;
}

async fn handle_disconnect(sessions: &Sessions, session_id: usize) {
    log::info!("disconneting session: {}", session_id);

    // Stream closed up, so remove from the user list
    sessions.write().await.remove(&session_id);
}

async fn handle_request(sessions: &Sessions, session_id: usize, request: WsRequest) {}
