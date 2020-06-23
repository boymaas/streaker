use warp::Filter;

mod page;
use page::index;
use page::login;

use maud;

fn render_html(html: maud::Markup) -> impl warp::reply::Reply {
    warp::reply::html(html.into_string())
}

pub async fn start() {
    let log = warp::log("streaker");

    // Match any request
    let index = warp::get()
        .and(warp::path::end())
        .map(index::page)
        .map(render_html);

    // Static files
    let statics = warp::path("static").and(warp::fs::dir("public"));

    // Login page
    let login = warp::get()
        .and(warp::path!["login"])
        .map(login::page)
        .map(render_html);

    let routes = login.or(index).or(statics).with(log);

    // since we will be running inside a docker container
    // our server should exit on a CTRL-C
    ctrlc::set_handler(move || {
        println!("received SIGINT! Exiting process");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
