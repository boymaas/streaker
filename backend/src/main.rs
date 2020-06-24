#![feature(proc_macro_hygiene)]
// #![deny(warnings)]

mod web;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // Match any request and return hello world!
    web::start().await;
}
