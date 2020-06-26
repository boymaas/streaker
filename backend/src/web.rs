use warp::Filter;

mod token;

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

    let routes = token_fetch.or(api).with(cors).with(log);

    // since we will be running inside a docker container
    // our server should exit on a CTRL-C
    ctrlc::set_handler(move || {
        println!("received SIGINT! Exiting process");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
