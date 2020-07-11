use anyhow::Result;
use sqlx::postgres::PgConnection;

#[derive(Debug, PartialEq)]
pub struct AccessNode {
    pub label: String,
    pub description: Option<String>,
}

impl AccessNode {
    pub async fn create(pool: &mut PgConnection, label: &str) -> Result<AccessNode> {
        let anode: AccessNode = sqlx::query_as!(
            AccessNode,
            "INSERT INTO anodes (label) VALUES ( $1 ) returning *",
            label,
        )
        .fetch_one(pool)
        .await?;
        Ok(anode)
    }

    pub async fn count(pool: &mut PgConnection) -> Result<i64> {
        let result = sqlx::query!("SELECT count(*) as count FROM anodes")
            .fetch_one(pool)
            .await?;
        Ok(result.count.map_or(0, |v| v))
    }
}
