use anyhow::Result;
use include_dir::{include_dir, Dir};
use sqlx_pg_migrate;

// Use include_dir! to include your migrations into your binary.
// The path here is relative to your cargo root.
static MIGRATIONS: Dir = include_dir!("migrations");

pub async fn migrate(db_url: &str) -> Result<(), sqlx_pg_migrate::Error> {
    sqlx_pg_migrate::migrate(&db_url, &MIGRATIONS).await
}
