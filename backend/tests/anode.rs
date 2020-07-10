use dotenv;
use pretty_env_logger;
use uuid::Uuid;
use warp::{http::Method, Filter};

use tokio::sync::{mpsc, RwLock};

// since this is not a lib we need to mod
// modules as in the main
use streaker::testdb::prepare_database;
use streaker::web;
use streaker::web::anode;
use streaker::web::ws;

#[tokio::test]
async fn test_login() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let sessions = ws::Sessions::default();
    let sessions_any = {
        let sessions = sessions.clone();
        warp::any().map(move || sessions.clone())
    };
    //
    let pool = prepare_database().await;
    let pool_any = warp::any().map(move || pool.clone());

    let uuid = Uuid::parse_str("be42b325-c990-43fa-a1e4-384aef09df1d").unwrap();
    let (tx, rx) = mpsc::unbounded_channel();
    sessions.write().await.insert(uuid, (None, tx));

    //
    let body = r#"
        {
           "access_node_name":"opesdentist",
           "claim":{
              "action":"checkin",
              "source":"login:be42b325-c990-43fa-a1e4-384aef09df1d",
              "exp":"Fri Jul 10 10:58:34 GMT 2020",
              "visitorid":"IhG87MWGA1cWxcT5e6AlX1xqYeP0k1UP"
           }
        }
        "#;
    let route = warp::any()
        .and(warp::body::json())
        .and(sessions_any)
        .and(pool_any)
        .and_then(anode::attribution);

    let res = warp::test::request()
        .method("POST")
        .header("content-length", body.len())
        .header("content-type", "application/json")
        .body(body)
        .reply(&route)
        .await;

    println!("{:?}", res);
    //
    // let res = warp::test::request().method("POST");
}
