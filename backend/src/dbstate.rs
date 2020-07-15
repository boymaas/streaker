use lazy_static::lazy_static;
use sqlx::error::DatabaseError;
use sqlx::postgres::{PgError, PgPool};

use crate::model::AccessNode;
use log;

lazy_static! {
    static ref ANODES: Vec<(&'static str, &'static str)> = vec![
        ("opessports", "https://access-node.opessports.com"),
        ("opesunite", "https://access-node.opesunite.io"),
        ("opesoutdoors", "https://access-node.opesoutdoors.com"),
        ("opesflights", "https://access-node.opesflights.com"),
        ("opeschurches", "https://access-node.opeschurches.com"),
        ("opesmusic", "https://access-node.opesmusic.com"),
        ("opesfitness", "https://access-node.opesfitness.com"),
        ("opeskids", "https://access-node.opeskids.com"),
        ("opesauto", "https://access-node.opesauto.com"),
        ("opescharity", "https://access-node.opescharity.com"),
        ("opesbaby", "https://access-node.opesbaby.com"),
        ("opesbrides", "https://access-node.opesbrides.com"),
        ("opescooking", "https://access-node.opescooking.com"),
        ("opesdating", "https://access-node.opesdating.com"),
        ("opesdentist", "https://access-node.opesdentist.com"),
        ("opesdining", "https://access-node.opesdining.com"),
        ("opeseducation", "https://access-node.opeseducation.com"),
        ("opesgames", "https://access-node.opesgames.com"),
        ("opesgaming", "https://access-node.opesgaming.com"),
        ("opeshomes", "https://access-node.opeshomes.com"),
        ("opeshotels", "https://access-node.opeshotels.com"),
        ("opesjobs", "https://access-node.opesjobs.io"),
        ("opesmoms", "https://access-node.opesmoms.com"),
        ("opesmovies", "https://access-node.opesmovies.com"),
        ("opespayments", "https://access-node.opespayments.com"),
        ("opespets", "https://access-node.opespets.com"),
        ("opesrewards", "https://access-node.opesrewards.com"),
        ("opesrides", "https://access-node.opesrides.com"),
        ("opessweeps", "https://access-node.opessweeps.com"),
        ("opesvending", "https://access-node.opesvending.com"),
        ("opesstudent", "https://access-node.opesstudent.com"),
    ];
}

fn get_db_error_code<'a>(e: &'a anyhow::Error) -> Option<&'a str> {
    e.source()?.downcast_ref::<PgError>()?.code()
}

pub async fn ensure(pool: &PgPool) {
    // since this is on startup, we want ot fail hard
    let mut conn = pool.acquire().await.expect("problem acquiring connection");
    for (anode, url) in ANODES.iter() {
        println!("{:?}", (anode, url));
        if let Err(e) = AccessNode::create(&mut conn, anode, url, 1.0).await {
            if let Some("23505") = get_db_error_code(&e) {
                // don't continue
            } else {
                panic!("{:?}", &e);
            }
        }
    }
}
