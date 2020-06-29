use futures::{FutureExt, StreamExt};
use log;
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};
use warp::Filter;

use serde_json::json;

mod token;

async fn handle_ws_connection(socket: warp::ws::WebSocket) {
    let (ws_tx, mut ws_rx) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // send something on connection
    tx.send(Ok(Message::text(json!({"connected": true}).to_string())))
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

pub async fn start() {
    let log = warp::log("streaker");

    let api = warp::any().map(|| "OPES Unite Streaker API");

    let token_path = warp::path!("api" / "v1" / "token" / "fetch");
    let token_fetch = warp::post().and(token_path).map(token::fetch);

    let cors_origin: &str =
        &dotenv::var("STREAKER_CORS_ORIGIN").expect("STREAKER_CORS_ORIGIN must be set");
    let cors = warp::cors()
        .allow_origin(cors_origin)
        .allow_header("Content-Type")
        .allow_method("GET")
        .allow_method("POST");

    let ws = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_ws_connection));

    let routes = ws.or(token_fetch).or(api).with(cors).with(log);

    // since we will be running inside a docker container
    // our server should exit on a CTRL-C
    ctrlc::set_handler(move || {
        println!("received SIGINT! Exiting process");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
