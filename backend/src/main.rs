use anyhow::Result;
use dotenv;

mod dbstate;
mod jwt;
mod model;
mod web;

#[cfg(test)]
mod testdb;

use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let database_url = dotenv::var("DATABASE_URL")?;

    log::info!("Connecting to database: {}", database_url);
    let pool = PgPool::builder()
        .max_size(10) // maximum number of connections in the pool
        .build(&database_url)
        .await?;

    // Ensure we are migated
    log::info!("Applying any pending migrations");
    streaker_migrate::migrate(&database_url).await?;

    // Ensure we have a valid database state on startup
    log::info!("Ensuring application state");
    dbstate::ensure(&pool).await;

    // Start the web service
    log::info!("Starting web service");
    web::start(pool).await;

    Ok(())
}
