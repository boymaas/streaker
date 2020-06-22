use warp::Filter;

mod page;
use page::index;

use maud;

fn render_html(html: maud::PreEscaped<String>) -> impl warp::reply::Reply {
    warp::reply::html(html.into_string())
}

pub async fn start() {
    let _log = warp::log("streaker");

    // Match any request and return hello world!
    let index = warp::get()
        .and(warp::path::end())
        .map(index::page)
        .map(render_html);

    // Static files
    let statics = warp::path("static").and(warp::fs::dir("public"));

    let routes = index.or(statics);
    // .with(log) to enable access log

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
