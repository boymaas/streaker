use lazy_static::lazy_static;
use sqlx::error::DatabaseError;
use sqlx::postgres::{PgError, PgPool};

use crate::model::AccessNode;
use log;

lazy_static! {
    static ref ANODES: Vec<&'static str> = vec!["opesdentist"];
}

fn get_db_error_code<'a>(e: &'a anyhow::Error) -> Option<&'a str> {
    e.source()?.downcast_ref::<PgError>()?.code()
}

pub async fn ensure(pool: &PgPool) {
    for anode in ANODES.iter() {
        if let Err(e) = AccessNode::create(pool, anode).await {
            if let Some("23505") = get_db_error_code(&e) {
            } else {
                panic!("{:?}", &e);
            }
        }
    }
}