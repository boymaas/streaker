// Now link the source of the attribution
// to the websocket channel, and generate
// a new token and send it over the websocket
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::PgPool;
use warp;
use warp::reply::Json;

use streaker_common::ws::WsResponse;

use crate::jwt;
use crate::model::member::Member;
use crate::web::ws::{send_response, Sessions};

use super::{reject, Attribution};

// when we receive an attribution, and we have a session
// which matches the suuid of the attribution. We can authenticate
// that session with the correct visitor id
pub async fn attribution_login(
    attr: Attribution,
    ws_sessions: Sessions,
    pool: PgPool,
    time: &DateTime<Utc>,
) -> Result<Json, warp::reject::Rejection> {
    log::info!("LOGIN: {:?}", attr);
    let mut conn = pool
        .acquire()
        .await
        .map_err(reject("problem acquiring connection"))?;
    if let Some((_, ws_channel)) = ws_sessions.read().await.get(&attr.claim.source.suuid) {
        // now we have the channel so we can generate a new authenticated token
        // and send the update over the channel.
        // make sure the token has the same suuid, as this is tied to the websocket.
        let auth_token =
            jwt::generate_authenticated_token(&attr.claim.source.suuid, &attr.claim.visitorid);
        send_response(ws_channel, &WsResponse::Attribution(auth_token));

        // now also send out member state
        // fetch member from database, and build the member state
        if let Ok(member) = Member::fetch(&mut conn, &attr.claim.visitorid).await {
            // we have a member in the database, so lets send it over
            send_response(ws_channel, &WsResponse::MemberState(member.into()));
        } else {
            // we do not have a member in the database lets create one
            if let Ok(member) = Member::add(&mut conn, &attr.claim.visitorid).await {
                send_response(ws_channel, &WsResponse::MemberState(member.into()));
            } else {
                // we could not create this member, we need to communicate this back
                // to the UI
                send_response(
                    ws_channel,
                    &WsResponse::Error("Could not create MemberState".into()),
                );
            };
        };

        Ok(warp::reply::json(&json!({"success": true})))
    } else {
        Err(warp::reject::not_found())
    }
}
