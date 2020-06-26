#![feature(proc_macro_hygiene)]
// #![deny(warnings)]

use dotenv::dotenv;

mod jwt;
mod web;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    // Match any request and return hello world!
    web::start().await;
}
