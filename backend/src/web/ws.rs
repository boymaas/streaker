use futures::{FutureExt, StreamExt};
use log;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::ws::Message;

use streaker_common::ws::{WsRequest, WsResponse};

pub type WsChannel = mpsc::UnboundedSender<Result<Message, warp::Error>>;
pub type Sessions = Arc<RwLock<HashMap<Uuid, WsChannel>>>;

fn send_response(tx: &mut WsChannel, ws_response: WsResponse) {
    tx.send(Ok(Message::text(
        serde_json::to_string(&ws_response).unwrap(),
    )))
    .unwrap();
}

pub async fn handle(sessions: Sessions, suuid: Uuid, socket: warp::ws::WebSocket) {
    // when we already have another tab open, we can send a message to the
    // previous open tab to close that one.

    let (ws_tx, mut ws_rx) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            log::error!("websocket send error: {}", e);
        }
    }));

    // store the sender in our sessions
    // if we had an open connection, close that one
    if let Some(mut old_tx) = sessions.write().await.insert(suuid, tx.clone()) {
        send_response(&mut old_tx, WsResponse::DoubleConnection);
    }

    log::info!("{:?}", sessions.read().await.keys());

    // send something on connection
    tx.send(Ok(Message::text(
        serde_json::to_string(&WsResponse::Connected).unwrap(),
    )))
    .unwrap();

    log::info!("Connection established suuid={}", suuid);
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(suuid={}) {}", suuid, e);
                break;
            }
        };

        // TODO: catch this
        match msg.to_str() {
            Ok(msg) => {
                if let Ok(request) = serde_json::from_str::<WsRequest>(msg) {
                    // do something with message
                    handle_request(&sessions, suuid, request).await;
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
    handle_disconnect(&sessions, suuid).await;
}

async fn handle_disconnect(sessions: &Sessions, session_id: Uuid) {
    log::info!("disconneting session: {}", session_id);

    // Stream closed up, so remove from the user list
    sessions.write().await.remove(&session_id);
}

async fn handle_request(sessions: &Sessions, session_id: Uuid, request: WsRequest) {
    match request {}
}
