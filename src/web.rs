use warp::Filter;

mod page;
use page::index;

use maud;

fn render_html(html: maud::PreEscaped<String>) -> impl warp::reply::Reply {
    warp::reply::html(html.into_string())
}

pub async fn start() {
    // Match any request and return hello world!
    let routes = warp::get()
        .and(warp::path::end())
        .map(index::page)
        .map(render_html);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
