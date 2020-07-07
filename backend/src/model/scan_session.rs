use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct ScanSession {
    pub uuid: Uuid,
    pub visitorid: String,
    pub begin: DateTime<Utc>,
}

pub struct Scan {
    pub scansession: Uuid,
    pub anode: String,
    pub tstamp: DateTime<Utc>,
}

impl ScanSession {
    pub fn new(visitorid: &str, begin: &DateTime<Utc>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            visitorid: visitorid.to_string(),
            begin: begin.clone(),
        }
    }

    pub async fn register_scan(&self, pool: &PgPool, anode: &str) -> Result<Scan> {
        let scan: Scan = sqlx::query_as!(
            Scan,
            "INSERT INTO scans (scansession,anode,tstamp) VALUES ( $1, $2, $3 ) returning *",
            self.uuid,
            anode,
            Utc::now()
        )
        .fetch_one(pool)
        .await?;
        Ok(scan)
    }

    // fetches current scansession, appropiate for server
    // time.
    pub async fn current(pool: &PgPool, visitorid: &str) -> Result<ScanSession> {
        // if we have one, check if it is still valid against
        // UTC time. Notice the ? to leave early on error.
        if let Some(session) = Self::latest(pool, visitorid).await? {
            Ok(session)
        } else {
            // we don't have one, so lets create one
            // aligned to 0:00 UTC
            let begin = &Utc::now().date().and_hms(0, 0, 0);
            Ok(Self::create(&pool, visitorid, begin).await?)
        }
    }

    pub async fn latest(pool: &PgPool, visitorid: &str) -> Result<Option<ScanSession>> {
        let session: Option<ScanSession> = sqlx::query_as!(
            ScanSession,
            "SELECT * FROM scansessions WHERE visitorid = $1 order by begin desc limit 1",
            visitorid
        )
        .fetch_optional(pool)
        .await?;
        Ok(session)
    }

    pub async fn create(
        pool: &PgPool,
        visitorid: &str,
        begin: &DateTime<Utc>,
    ) -> Result<ScanSession> {
        let uuid = Uuid::new_v4();
        let session: ScanSession = sqlx::query_as!(
            ScanSession,
            "INSERT INTO scansessions (visitorid,uuid,begin) VALUES ( $1, $2, $3 ) returning *",
            visitorid,
            uuid,
            *begin
        )
        .fetch_one(pool)
        .await?;
        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::ScanSession;
    use crate::model::AccessNode;
    use crate::model::Member;
    use crate::testdb::prepare_database;

    #[tokio::test]
    async fn test_current() {
        // drops and migrates the test database
        let pool = prepare_database().await;

        let visitorid = "VISITORID";
        let _member = Member::add(&pool, visitorid).await;

        // Now create our member
        let session = ScanSession::current(&pool, visitorid).await.unwrap();
        assert_eq!(session.visitorid, visitorid);

        // Now register a scan
        let anode = AccessNode::create(&pool, "opesdentist").await.unwrap();
        let scan = session.register_scan(&pool, "opesdentist").await.unwrap();

        assert_eq!(scan.scansession, session.uuid);
    }
}
