#![feature(proc_macro_hygiene)]
#![deny(warnings)]
use warp::Filter;

mod page;
use page::index;

use maud;

fn render_html(html: maud::PreEscaped<String>) -> impl warp::reply::Reply {
    warp::reply::html(html.into_string())
}

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| index::page()).map(render_html);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
