use std::sync::Once;

use dotenv;
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

#[tokio::test]
async fn test_streaker_client() {
    let app = prepare_test_app().await;

    let mut client = StreakerClient::new(app);

    // Connect, this will get our first
    // unauthenticated token
    client.connect().await;
    // Now connect to the websocket
    // using our unauthenticated token
    client.ws_connect().await;

    // Now simulate our accessnode registering the login
    // scan.
    // This will send an attribution to the client containing
    // and authenticated token. With the current state of the
    // authenticated visitor id.
    client
        .post_attribution_login("opesdentist", "IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP")
        .await;

    // now simulate a scan coming through to our application
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
    let mut conn = client.streaker_app.pool.acquire().await.unwrap();
    let access_node_count = AccessNode::count(&mut conn).await.unwrap();

    // the scan session should have covered all the registered
    // access nodes!
    assert_eq!(
        access_node_count,
        client.scan_session_state.unwrap().count as i64
    );

    // This is a minimal roundtrip test
}
