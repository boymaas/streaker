use std::sync::Once;

use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use dotenv;
use log;
use pretty_env_logger;

use streaker_common::ws::{MemberState, ScanSessionState, StreakState, WsResponse};

// since this is not a lib we need to mod
// modules as in the main
use streaker::model::AccessNode;
use streaker::streaker_client::StreakerClient;
use streaker::testdb::prepare_database;
use streaker::web::StreakerApp;

static INIT: Once = Once::new();
async fn prepare_test_app() -> StreakerApp {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        pretty_env_logger::init();
    });

    // build and migrate database
    let pool = prepare_database().await;

    StreakerApp::new(pool)
}

async fn prepare_test_client_and_login(
    anode: &str,
    visitorid: &str,
    timefn: fn() -> DateTime<Utc>,
) -> StreakerClient {
    let app = prepare_test_app().await;

    // create client, and set the time
    let mut client = StreakerClient::new(app);
    client.set_time(timefn);

    // Connect, this will get our first
    // unauthenticated token
    client.connect().await;
    // Now connect to the websocket
    // using our unauthenticated token
    client.ws_connect().await;

    // We can use the ? operator to check for
    // errors, beats the unwraps
    // Err(anyhow::anyhow!("Test exit"))?;

    // Now simulate our accessnode registering the login
    // scan.
    // This will send an attribution to the client containing
    // and authenticated token. With the current state of the
    // authenticated visitor id.
    client.post_attribution_login(anode, visitorid).await;

    client
}

#[tokio::test]
async fn test_streaker_client() -> Result<()> {
    let app = prepare_test_app().await;

    let mut client = StreakerClient::new(app);

    // Connect, this will get our first
    // unauthenticated token
    client.connect().await;
    // Now connect to the websocket
    // using our unauthenticated token
    client.ws_connect().await;

    // We can use the ? operator to check for
    // errors, beats the unwraps
    // Err(anyhow::anyhow!("Test exit"))?;

    // Now simulate our accessnode registering the login
    // scan.
    // This will send an attribution to the client containing
    // and authenticated token. With the current state of the
    // authenticated visitor id.
    client
        .post_attribution_login("opesdentist", "IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP")
        .await;

    // now simulate scans coming through our application
    // this will, register a scan, and we will receive the new
    // states from our websocket connection.
    //
    // we are just checking the scan_session_state if there
    // are access nodes to be scanned, as long it is not
    // None we keep on going. We expect the scans to match
    // the amount of access nodes.
    while let Some(ScanSessionState {
        next_anode: Some(anode),
        ..
    }) = &client.scan_session_state
    {
        // NOTE the colone is necessary as I cannot borrow immutable and mutable
        // at the same time. As I am not referencing the borrow anymore. I can access
        // the struct immutable doing the post_attribution_scan.
        let anode = anode.clone();
        client
            .post_attribution_scan(&anode, "IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP")
            .await;
    }

    // So lets count the number of access nodes
    // TODO: make accessor to get db_connection from the pool
    let mut conn = client.streaker_app.pool.acquire().await?;
    let access_node_count = AccessNode::count(&mut conn).await?;

    // the scan session should have covered all the registered
    // access nodes!
    assert_eq!(
        access_node_count,
        client.scan_session_state.unwrap().count as i64
    );

    // This is a minimal roundtrip test
    Ok(())
}

fn assert_member_state(
    client: &StreakerClient,
    streak_current: i32,
    streak_bucket: i32,
    balance: f64,
) {
    let member_state = client.member_state.as_ref().unwrap();
    assert_eq!(member_state.streak_current, streak_current);
    assert_eq!(member_state.streak_bucket, streak_bucket);
    assert_eq!(member_state.balance, balance);
}

fn assert_scan_session_state(client: &StreakerClient, count: u16, begin: DateTime<Utc>) {
    let scan_session_state = client.scan_session_state.as_ref().unwrap();
    assert_eq!(scan_session_state.count, count);
    assert_eq!(scan_session_state.total, 31);
    assert_eq!(scan_session_state.begin, begin.date().and_hms(0, 0, 0));
}

fn assert_streak_state(
    client: &StreakerClient,
    streak_current: i32,
    streak_bucket: i32,
    streak_missed: i32,
    bucket: i32,
    mining_ratio: f64,
    days_since: i32,
) {
    let streak_state = client.streak_state.as_ref().unwrap();
    assert_eq!(streak_state.streak_current, streak_current);
    assert_eq!(streak_state.streak_bucket, streak_bucket);
    assert_eq!(streak_state.streak_missed, streak_missed);
    assert_eq!(streak_state.bucket, bucket);
    assert_eq!(streak_state.mining_ratio, mining_ratio);
    assert_eq!(streak_state.days_since_last_scan, days_since);
}

#[tokio::test]
async fn test_correct_registration_of_streaks() -> Result<()> {
    fn now() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 01).and_hms(12, 0, 0)
    }

    log::info!("First Login");
    let visitorid = "IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP";
    let mut client = prepare_test_client_and_login("opesdentist", visitorid, now).await;

    // Start with a 0 streak/bucket and 0 streak count
    assert_member_state(&client, 0, 0, 0.0);
    assert_scan_session_state(&client, 0, now());
    assert_streak_state(&client, 0, 0, 0, 0, 0.0025, 0);

    log::info!("First scan");
    // now the client scans today, one scan
    client.post_attribution_scan("opesgames", visitorid).await;

    // first scan as such streaks stat 0
    assert_member_state(&client, 0, 0, 0.0025);
    assert_scan_session_state(&client, 1, now());
    assert_streak_state(&client, 0, 0, 0, 0, 0.0025, 0);

    log::info!("Second scan same day");
    // now the client scans again
    client.post_attribution_scan("opesdentist", visitorid).await;

    // new scan session state with 1
    assert_member_state(&client, 0, 0, 0.0050);
    assert_scan_session_state(&client, 2, now());
    assert_streak_state(&client, 0, 0, 0, 0, 0.0025, 0);

    // ===> now lets move to the next day
    log::info!("New scan day 1");
    fn now_plus_one() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 02).and_hms(12, 0, 0)
    }
    client.set_time(now_plus_one);

    // now the client scans again
    client.post_attribution_scan("opesgames", visitorid).await;

    // this was a day after, so we want to see a streak of 1
    assert_member_state(&client, 1, 1, 0.0075);
    assert_scan_session_state(&client, 1, now_plus_one());
    assert_streak_state(&client, 1, 1, 0, 0, 0.0025, 1);

    // ===> now lets move to the next day
    log::info!("New scan day 2");
    fn now_plus_two() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 03).and_hms(12, 0, 0)
    }
    client.set_time(now_plus_two);

    // now the client scans again
    client.post_attribution_scan("opesgames", visitorid).await;

    // this was a day after, so we want to see a streak of 2
    assert_member_state(&client, 2, 2, 0.010);
    assert_scan_session_state(&client, 1, now_plus_two());
    assert_streak_state(&client, 2, 2, 0, 0, 0.0025, 1);

    // ===> now lets move to the next day
    log::info!("New scan day 3");
    fn now_plus_three() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 04).and_hms(12, 0, 0)
    }
    client.set_time(now_plus_three);

    // now the client scans again
    client.post_attribution_scan("opesgames", visitorid).await;

    // this was a day after, so we want to see a streak of 2
    assert_member_state(&client, 3, 3, 0.0125);
    assert_scan_session_state(&client, 1, now_plus_three());
    assert_streak_state(&client, 3, 3, 0, 0, 0.0025, 1);

    // ===> now lets move to the next day, now we should see
    // a bucket shift
    log::info!("New scan day 4, bucket shift");
    fn now_plus_four() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 05).and_hms(12, 0, 0)
    }
    client.set_time(now_plus_four);

    // now the client scans again
    client.post_attribution_scan("opesgames", visitorid).await;

    // this was a day after, so we want to see and increase
    // in streak size, and in this case, also an increase in bucket
    assert_member_state(&client, 4, 4, 0.0155);
    assert_scan_session_state(&client, 1, now_plus_four());
    assert_streak_state(&client, 4, 4, 0, 1, 0.003, 1);

    // ==> lets do one more scan
    log::info!("New scan day 5");
    fn now_plus_5() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 06).and_hms(12, 0, 0)
    }
    client.set_time(now_plus_5);

    // now the client scans again
    client.post_attribution_scan("opesgames", visitorid).await;

    // ==> lets do one more scan
    log::info!("New scan day 6");
    fn now_plus_6() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 07).and_hms(12, 0, 0)
    }
    client.set_time(now_plus_6);

    // now the client scans again
    client.post_attribution_scan("opesgames", visitorid).await;

    assert_member_state(&client, 6, 6, 0.022);
    assert_scan_session_state(&client, 1, now_plus_6());
    assert_streak_state(&client, 6, 6, 0, 2, 0.0034999999999999996, 1);

    // ==> lets do one more scan
    log::info!("New scan day 7");
    fn now_plus_7() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 08).and_hms(12, 0, 0)
    }
    client.set_time(now_plus_7);

    // now the client scans again
    client.post_attribution_scan("opesgames", visitorid).await;

    assert_member_state(&client, 7, 7, 0.0255);
    assert_scan_session_state(&client, 1, now_plus_7());
    assert_streak_state(&client, 7, 7, 0, 2, 0.0034999999999999996, 1);

    // Se we build up our streak to the second bucket, and we have a streak
    // of 7. Now lets miss one day. First we login to check the state.
    log::info!("New LOGIN day 9, we missed a day!");
    fn now_plus_9() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 10).and_hms(12, 0, 0)
    }
    client.set_time(now_plus_9);

    // now the client logs in
    client.post_attribution_login("opesgames", visitorid).await;

    // member_state is still in the expected state
    assert_member_state(&client, 7, 7, 0.0255);
    // a new scan session state has been created with a state of 0
    assert_scan_session_state(&client, 0, now_plus_9());
    // streat state now communicates the user missed a streak, we do this
    // so we can communicate this on the client side.
    // we lost our streak :(
    assert_streak_state(&client, 0, 4, 1, 1, 0.003, 2);

    // so now lets peform a scan, we exect the member state to be updated
    log::info!("New SCAN day 9, we missed a day!");
    client.post_attribution_scan("opesgames", visitorid).await;

    // member_state is still in the expected state
    assert_member_state(&client, 0, 4, 0.0285);
    assert_scan_session_state(&client, 1, now_plus_9());
    // streat state now communicates the user missed a streak, we do this
    // so we can communicate this on the client side.
    // we lost our streak :(
    assert_streak_state(&client, 0, 4, 0, 1, 0.003, 0);

    Ok(())
}

#[tokio::test]
async fn test_double_scans_blocked() -> Result<()> {
    fn now() -> DateTime<Utc> {
        Utc.ymd(2020, 01, 01).and_hms(12, 0, 0)
    }

    log::info!("First Login");
    let visitorid = "IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP";
    let mut client = prepare_test_client_and_login("opesdentist", visitorid, now).await;

    log::info!("First scan");
    // now the client scans today, one scan
    client.post_attribution_scan("opesgames", visitorid).await;

    // TODO: double check that this actually fails
    //       and we should handle this case gracefully
    // client.post_attribution_scan("opesgames", visitorid).await;

    Ok(())
}
