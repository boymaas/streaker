use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgConnection;
use uuid::Uuid;

use streaker_common::ws;
use streaker_common::ws::ScanSessionState;

use crate::model::anode::AccessNode;
use crate::model::scan::Scan;

#[derive(Debug, PartialEq)]
pub struct ScanSession {
    pub uuid: Uuid,
    pub visitorid: String,
    pub begin: DateTime<Utc>,
}

impl ScanSession {
    pub fn new(visitorid: &str, begin: &DateTime<Utc>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            visitorid: visitorid.to_string(),
            begin: begin.clone(),
        }
    }

    pub async fn register_scan(
        &self,
        pool: &mut PgConnection,
        anode: &str,
        time: &DateTime<Utc>,
    ) -> Result<Scan> {
        let scan: Scan = sqlx::query_as!(
            Scan,
            "INSERT INTO scans (scansession,anode,tstamp) VALUES ( $1, $2, $3 ) returning *",
            self.uuid,
            anode,
            *time
        )
        .fetch_one(pool)
        .await?;
        Ok(scan)
    }

    // fetches current scansession, appropiate for server
    // time.
    pub async fn current(
        pool: &mut PgConnection,
        visitorid: &str,
        time: &DateTime<Utc>,
    ) -> Result<ScanSession> {
        // aligned to 0:00 UTC
        let begin = time.clone().date().and_hms(0, 0, 0);

        // if we have one, check if it is still valid against
        // UTC time. Notice the ? to leave early on error.
        // TODO: this can be made prettyer
        if let Some(session) = Self::latest(pool, visitorid).await? {
            if session.begin == begin {
                return Ok(session);
            }
        }
        // we don't have one, or the one we have has been expired
        // so lets create one
        Ok(Self::create(pool, visitorid, &begin).await?)
    }

    // build the scansession state, used to send to the client
    // to render the scan! page
    pub async fn scan_session_state(
        &self,
        pool: &mut PgConnection,
        time: &DateTime<Utc>,
    ) -> Result<ScanSessionState> {
        let scan_session = Self::current(pool, &self.visitorid, time).await?;

        // NOTE: https://github.com/launchbadge/sqlx/issues/257
        // see the reborrows here. Since the fetch method is
        // generic over its parameter (allowing both Pool and Connection)
        // to be passed in.
        let total = sqlx::query!("select count(*) as count from anodes")
            .fetch_one(&mut *pool)
            .await?;

        let scans_performed = sqlx::query!(
            "select count(*) as count from scans where scansession = $1",
            scan_session.uuid
        )
        .fetch_one(&mut *pool)
        .await?;

        let next_anode: Option<AccessNode> = sqlx::query_as!(
            AccessNode,
            r#"select * from anodes
                 where 
                   anodes.label NOT IN (select anode from scans where scansession = $1)"#,
            scan_session.uuid
        )
        .fetch_optional(&mut *pool)
        .await?;

        // build up the scansession state
        let scan_session_state = ScanSessionState {
            uuid: scan_session.uuid,
            count: scans_performed.count.unwrap() as u16,
            total: total.count.unwrap() as u16,
            next_anode: next_anode.map(|anode| anode.into()),
            begin: scan_session.begin,
        };

        Ok(scan_session_state)
    }

    pub async fn latest(pool: &mut PgConnection, visitorid: &str) -> Result<Option<ScanSession>> {
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
        pool: &mut PgConnection,
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
