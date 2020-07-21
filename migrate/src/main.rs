use anyhow::Result;
use dotenv;
use tokio;

// Somewhere, probably in main, call the migrate function with your DB URL
// and the included migrations.
#[tokio::main]
async fn main() -> Result<()> {
    let db_url = dotenv::var("DATABASE_URL").unwrap();

    ::streaker_migrate::migrate(&db_url).await?;

    Ok(())
}
