use anyhow::Result;
use dotenv;
use include_dir::{include_dir, Dir};
use sqlx_pg_migrate;
use tokio;

// Use include_dir! to include your migrations into your binary.
// The path here is relative to your cargo root.
static MIGRATIONS: Dir = include_dir!("migrations");

pub async fn migrate(db_url: &str) -> Result<(), sqlx_pg_migrate::Error> {
    sqlx_pg_migrate::migrate(&db_url, &MIGRATIONS).await
}

// Somewhere, probably in main, call the migrate function with your DB URL
// and the included migrations.
#[tokio::main]
async fn main() -> Result<()> {
    let db_url = dotenv::var("DATABASE_URL").unwrap();

    migrate(&db_url).await?;

    Ok(())
}
