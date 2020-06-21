#![feature(proc_macro_hygiene)]
#![deny(warnings)]

mod web;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    web::start().await;
}
