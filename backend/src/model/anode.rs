use anyhow::Result;
use sqlx::postgres::PgConnection;

use streaker_common::ws;

#[derive(Debug, PartialEq)]
pub struct AccessNode {
    pub label: String,
    pub url: String,
    pub weight: f64,
    pub description: Option<String>,
}

impl AccessNode {
    pub async fn create(
        pool: &mut PgConnection,
        label: &str,
        url: &str,
        weight: f64,
    ) -> Result<AccessNode> {
        let anode: AccessNode = sqlx::query_as!(
            AccessNode,
            "INSERT INTO anodes (label, url) VALUES ($1, $2) returning *",
            label,
            url
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

impl Into<ws::AccessNode> for AccessNode {
    fn into(self) -> ws::AccessNode {
        ws::AccessNode {
            label: self.label,
            url: self.url,
            weight: self.weight,
            description: self.description,
        }
    }
}
