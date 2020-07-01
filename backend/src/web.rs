use warp::Filter;

mod anode;
mod token;
mod ws;

use uuid::Uuid;

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

    // build a filter which clones our Arc on each
    // new connection request.
    let websocket_sessions = ws::Sessions::default();
    let websocket_sessions_any = warp::any().map(move || websocket_sessions.clone());
    let websocket = warp::path!("ws" / Uuid)
        .and(warp::ws())
        .and(websocket_sessions_any.clone())
        .map(|suid: Uuid, ws: warp::ws::Ws, sessions| {
            ws.on_upgrade(move |socket| ws::handle(sessions, suid, socket))
        });

    // the attribution from the access node
    let attribution = warp::post()
        .and(warp::path!("api" / "v1" / "anode" / "attribution"))
        // NOTE: how the type system works here
        // I specify the json body here, and it magically deserialises
        // in the signature of the map beneath
        .and(warp::body::json())
        .and(websocket_sessions_any.clone())
        .and_then(anode::attribution);

    let routes = websocket
        .or(attribution)
        .or(token_fetch)
        .or(api)
        .with(cors)
        .with(log);

    // since we will be running inside a docker container
    // our server should exit on a CTRL-C
    ctrlc::set_handler(move || {
        println!("received SIGINT! Exiting process");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
