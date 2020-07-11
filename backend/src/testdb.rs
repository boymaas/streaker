use crate::migrate;
use dotenv;
use sqlx::postgres::PgPool;
use sqlx::{Connect, Executor, PgConnection};
use url::Url;

use crate::dbstate;

fn testbase_url() -> String {
    dotenv::var("TESTBASE_URL").unwrap()
}

fn conn_url() -> Url {
    let conn_url = testbase_url();
    Url::parse(&conn_url).unwrap()
}

pub async fn test_db_pool() -> PgPool {
    PgPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(&testbase_url())
        .await
        .unwrap()
}

// pub async fn test_db_connection() -> PgConnection {
//     Connect::connect(testbase_url()).await.unwrap()
// }

pub fn test_db_name() -> String {
    let conn_url = conn_url();
    (&conn_url.path()[1..]).to_string()
}

pub async fn clean_database() {
    // connect to our template1 databse to drop
    // other connections
    let template1_conn = conn_url().join("../template1").unwrap().to_string();

    // println!("MPL {}", template1_conn);
    let mut client: PgConnection = Connect::connect(template1_conn).await.unwrap();

    let dbname = test_db_name();

    // println!("DBNAME {}", dbname);

    // drop connections
    sqlx::query("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1")
        .bind(&dbname)
        .execute(&mut client)
        .await
        .unwrap();

    // drop and recreate database
    sqlx::query(&format!("DROP DATABASE IF EXISTS {}", dbname))
        .execute(&mut client)
        .await
        .unwrap();

    sqlx::query(&format!("CREATE DATABASE {}", dbname))
        .execute(&mut client)
        .await
        .unwrap();
}

pub async fn migrate_database() {
    migrate::migrate(&testbase_url()).await.unwrap();
}

pub async fn prepare_database() -> PgPool {
    clean_database().await;
    migrate_database().await;
    let pool = test_db_pool().await;
    dbstate::ensure(&pool).await;
    pool
}
