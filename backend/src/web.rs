use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use warp::filters::BoxedFilter;
use warp::reply::Json;
use warp::{Filter, Rejection, Reply};

pub mod anode;
pub mod token;
pub mod ws;

use uuid::Uuid;

pub struct StreakerApp {
    pub pool: PgPool,
    pub sessions: ws::Sessions,
}

impl StreakerApp {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            sessions: ws::Sessions::default(),
        }
    }

    pub fn route_db_pool_any(
        &self,
    ) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
        let pool = self.pool.clone();
        warp::any().map(move || pool.clone())
    }

    pub fn route_sessions_any(
        &self,
    ) -> impl Filter<Extract = (ws::Sessions,), Error = Infallible> + Clone {
        let sessions = self.sessions.clone();
        warp::any().map(move || sessions.clone())
    }

    pub fn route_api_token_fetch(
        &self,
        timefn: fn() -> DateTime<Utc>,
    ) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
        let token_path = warp::path!("api" / "v1" / "token" / "fetch");
        warp::post()
            .and(token_path)
            .and(warp::any().map(timefn))
            .map(token::fetch)
    }

    pub fn route_api_anode_attribution(
        &self,
        timefn: fn() -> DateTime<Utc>,
    ) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
        warp::post()
            .and(warp::path!("api" / "v1" / "anode" / "attribution"))
            // NOTE: how the type system works here
            // I specify the json body here, and it magically deserialises
            // in the signature of the map beneath
            .and(warp::body::json())
            .and(self.route_sessions_any())
            .and(self.route_db_pool_any())
            .and(warp::any().map(timefn))
            .and_then(anode::attribution)
    }

    pub fn route_ws(
        &self,
        timefn: fn() -> DateTime<Utc>,
    ) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        warp::path!("ws" / String)
            .and(warp::ws())
            .and(self.route_sessions_any())
            .and(self.route_db_pool_any())
            .map(move |token: String, ws: warp::ws::Ws, sessions, pool| {
                ws.on_upgrade(move |socket| ws::handle(sessions, pool, token, socket, timefn))
            })
    }

    // https://github.com/seanmonstar/warp/issues/53#issuecomment-412367454
    pub fn routes(
        &self,
        timefn: fn() -> DateTime<Utc>,
    ) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        let log = warp::log("streaker");

        let cors_origin: &str =
            &dotenv::var("STREAKER_CORS_ORIGIN").expect("STREAKER_CORS_ORIGIN must be set");
        let cors = warp::cors()
            .allow_origin(cors_origin)
            .allow_header("Content-Type")
            .allow_method("GET")
            .allow_method("OPTION")
            .allow_method("POST");

        self.route_ws(timefn)
            .or(warp::path("healthy").and(warp::any().map(|| "Ok")))
            .or(self.route_api_anode_attribution(timefn))
            .or(self.route_api_token_fetch(timefn))
            .with(cors)
            .with(log)
    }
}

pub async fn start(pool: PgPool) {
    let app = StreakerApp::new(pool);
    let routes = app.routes(Utc::now);

    // since we will be running inside a docker container
    // our server should exit on a CTRL-C
    ctrlc::set_handler(move || {
        println!("received SIGINT! Exiting process");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let port = dotenv::var("PORT")
        .expect("PORT must be defined")
        .parse()
        .expect("could not parse PORT");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
