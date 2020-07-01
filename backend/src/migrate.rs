use refinery::config::{Config, ConfigDbType};
use std::env;
use url::Url;

mod migrations;

// Migration is run synchronously since sqlx is not natively
// supported by Refinery.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let db_url = Url::parse(&dotenv::var("STREAKER_DATABASE_URL")?)?;
    let db_host = db_url.host().expect("host to be defined");
    let db_port = db_url.port().expect("port to be defined");
    let db_user = db_url.username();
    let db_pass = db_url.password().expect("password to be defined");
    let db_name = db_url.path();

    // Refinery and sqlx seem to be using configuration key
    // differently. Thus we need to set it like so
    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_user(&db_user)
        .set_db_pass(&db_pass)
        .set_db_host(&db_host.to_string())
        .set_db_port(&db_port.to_string())
        .set_db_name(&db_name[1..]);
    println!("Running migrations");
    migrations::runner().run(&mut conn)?;
    Ok(())
}
