use futures::{FutureExt, StreamExt};
use log;
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};
use warp::Filter;

use serde_json::json;

use streaker_common::ws::{WsRequest, WsResponse};

pub async fn handle(socket: warp::ws::WebSocket) {
    let (ws_tx, mut ws_rx) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

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
                eprintln!("websocket error {}", e);
                break;
            }
        };
        // do something with message
    }
    // handle disconnect
}
