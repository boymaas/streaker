#![feature(proc_macro_hygiene)]
// #![deny(warnings)]

use anyhow::Result;
use dotenv::dotenv;

mod jwt;
mod web;

use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let pool = PgPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(&dotenv::var("STREAKER_DATABASE_URL")?)
        .await?;

    // Match any request and return hello world!
    web::start(&pool).await;

    Ok(())
}
