use sqlx::postgres::PgPool;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

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

    // https://github.com/seanmonstar/warp/issues/53#issuecomment-412367454
    pub fn routes(&self) -> BoxedFilter<(impl Reply,)> {
        let log = warp::log("streaker");

        let api = warp::any().map(|| "OPES Unite Streaker API");

        let token_path = warp::path!("api" / "v1" / "token" / "fetch");
        let token_fetch = warp::post().and(token_path).map(token::fetch);

        let cors_origin: &str =
            &dotenv::var("STREAKER_CORS_ORIGIN").expect("STREAKER_CORS_ORIGIN must be set");
        let cors = warp::cors()
            .allow_origin(cors_origin)
            .allow_header("Content-Type")
            .allow_method("GET")
            .allow_method("POST");

        // db_pool arc copy filter
        let db_pool_any = {
            let pool = self.pool.clone();
            warp::any().map(move || pool.clone())
        };

        // build a filter which clones our Arc on each
        // new connection request.
        let websocket_sessions_any = {
            let sessions = self.sessions.clone();
            warp::any().map(move || sessions.clone())
        };

        let websocket = warp::path!("ws" / String)
            .and(warp::ws())
            .and(websocket_sessions_any.clone())
            .and(db_pool_any.clone())
            .map(|token: String, ws: warp::ws::Ws, sessions, pool| {
                ws.on_upgrade(move |socket| ws::handle(sessions, pool, token, socket))
            });

        // the attribution from the access node
        let attribution = warp::post()
            .and(warp::path!("api" / "v1" / "anode" / "attribution"))
            // NOTE: how the type system works here
            // I specify the json body here, and it magically deserialises
            // in the signature of the map beneath
            .and(warp::body::json())
            .and(websocket_sessions_any.clone())
            .and(db_pool_any.clone())
            .and_then(anode::attribution);

        websocket
            .or(attribution)
            .or(token_fetch)
            .or(api)
            .with(cors)
            .with(log)
            .boxed()
    }
}

pub async fn start(pool: PgPool) {
    let app = StreakerApp::new(pool);
    let routes = app.routes();

    // since we will be running inside a docker container
    // our server should exit on a CTRL-C
    ctrlc::set_handler(move || {
        println!("received SIGINT! Exiting process");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
