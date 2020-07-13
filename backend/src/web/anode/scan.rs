// Now link the source of the attribution
// to the websocket channel, and generate
// a new token and send it over the websocket
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::{PgConnection, PgPool};
use warp;
use warp::reply::Json;

use streaker_common::streak_logic::StreakLogic;
use streaker_common::ws::WsResponse;

use crate::model::member::Member;
use crate::model::scan::Scan;
use crate::model::scan_session::ScanSession;
use crate::web::ws::{send_response, Sessions};

use super::{reject, Attribution};

// Handle the scan attribution, looking up the scansession for the
// logged in member.
async fn attribution_scan_inner(
    attr: Attribution,
    ws_sessions: Sessions,
    conn: &mut PgConnection,
    time: &DateTime<Utc>,
) -> Result<Json> {
    // lets get the visitorid from this ws_connection, this visitorid
    // has been set on a websocket connection with an authenticated token.
    if let Some((Some(visitorid), ws_channel)) =
        ws_sessions.read().await.get(&attr.claim.source.suuid)
    {
        let mut member = Member::fetch(conn, &visitorid).await?;

        // now lets build our StreakerState
        // for this we need our last registered scan. And some fields
        // of our member
        let last_scan = Scan::last_scan(conn, visitorid).await?;
        let streak_logic = StreakLogic::new(
            member.streak_current,
            member.streak_bucket,
            last_scan.map(|ls| ls.tstamp),
        );

        let streak_state = streak_logic.evaluate(time);

        // if we missed a streak last time, update
        // our member to reflect the penalty. Essentially
        // by updating the streak_{bucket,current}
        //
        // streak_bucket contains information at which
        // bucket the member is, and current is for motivational
        // purposes.

        // we missed a streak, as such apply penalty
        if streak_state.streak_missed > 0 {
            member
                .update_streak_info(
                    conn,
                    streak_state.streak_current,
                    streak_state.streak_bucket,
                )
                .await?;
        } else if streak_state.days_since_last_scan == 1 {
            // we are scanning in the 24-48 hours after the scan
            // window of our last scan, as such we earned a streak!
            member
                .update_streak_info(
                    conn,
                    streak_state.streak_current + 1,
                    streak_state.streak_bucket + 1,
                )
                .await?;
        }
        // now we have update our member state. If we haven't updated
        // it we just send the state as is.
        send_response(ws_channel, &WsResponse::MemberState(member.into()));

        // TODO: link them together &attr.claim.visitorid
        // Find our current scan sesssion, this could be a new one
        let scan_session = ScanSession::current(conn, visitorid, time).await?;

        // and register our scan, effectively setting last scan to now
        scan_session
            .register_scan(conn, &attr.access_node_name, time)
            .await?;

        // now generating a new scan session state based on the newly registered
        // scan. As now we need a new next-anode to scan.
        let scan_session_state = scan_session.scan_session_state(conn, time).await?;
        send_response(
            &ws_channel,
            &WsResponse::ScanSessionState(scan_session_state),
        );

        // now send the new streak state over the websocket
        send_response(&ws_channel, &WsResponse::StreakState(streak_state));

        Ok(warp::reply::json(&json!({"success": true})))
    } else {
        Err(anyhow::anyhow!("Session with visitorID not present!"))
    }
}

pub async fn attribution_scan(
    attr: Attribution,
    ws_sessions: Sessions,
    pool: PgPool,
    time: &DateTime<Utc>,
) -> Result<Json, warp::reject::Rejection> {
    log::info!("SCAN: {:?}", attr);

    // TODO: the pattern here is warp filters need to return
    // a warp reply or reject, but errors do not map to the reject.
    // We need to find a pattern for this.
    //
    // The introcution of the inner is just to make the map_err happen
    // on one spot.
    let mut transaction = pool
        .begin()
        .await
        .map_err(reject("Could not start transaction"))?;

    let result = attribution_scan_inner(attr, ws_sessions, &mut transaction, time)
        .await
        .map_err(reject("problem executing attribution"))?;

    transaction
        .commit()
        .await
        .map_err(reject("Could not commit transaction"))?;

    Ok(result)
}
