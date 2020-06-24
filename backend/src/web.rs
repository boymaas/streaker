use warp::Filter;

pub async fn start() {
    let log = warp::log("streaker");

    let api = warp::any().map(|| "OPES Unite Streaker API");

    let routes = api.with(log);

    // since we will be running inside a docker container
    // our server should exit on a CTRL-C
    ctrlc::set_handler(move || {
        println!("received SIGINT! Exiting process");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
