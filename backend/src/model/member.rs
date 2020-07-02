use anyhow::Result;
use sqlx::postgres::PgPool;

#[derive(Debug, Default, PartialEq)]
pub struct Member {
    visitorid: String,
    bucket: i32,
    streak_total: i32,
    streak_bucket: i32,
    balance: f64,
    email: Option<String>,
}

impl Member {
    pub fn new(visitorid: &str) -> Self {
        Self {
            visitorid: visitorid.to_string(),
            ..Self::default()
        }
    }

    pub async fn add(pool: &PgPool, visitorid: &str) -> Result<Member> {
        let member: Member = sqlx::query_as!(
            Member,
            "INSERT INTO members (visitorid) VALUES ( $1 ) returning *",
            visitorid
        )
        .fetch_one(pool)
        .await?;
        Ok(member)
    }
}

#[cfg(test)]
mod tests {
    use super::Member;
    use crate::testdb::prepare_database;

    #[tokio::test]
    async fn member_add() {
        // drops and migrates the test database
        let pool = prepare_database().await;

        // Now create our member
        let member = Member::add(&pool, "VISITORID").await.unwrap();
        assert_eq!(member, Member::new("VISITORID"));
    }
}
