use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgConnection;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct Scan {
    pub scansession: Uuid,
    pub anode: String,
    pub tstamp: DateTime<Utc>,
    pub skipped: bool,
}

impl Scan {
    pub async fn last_scan(pool: &mut PgConnection, visitorid: &str) -> Result<Option<Scan>> {
        let scan: Option<Scan> = sqlx::query_as!(
            Scan,
            r#"SELECT scans.* FROM scansessions 
                INNER JOIN scans ON scansessions.uuid = scans.scansession
                WHERE scansessions.visitorid = $1 
                ORDER BY tstamp DESC LIMIT 1"#,
            visitorid
        )
        .fetch_optional(pool)
        .await?;
        Ok(scan)
    }
}
