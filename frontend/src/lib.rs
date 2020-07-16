#![recursion_limit = "512"]
use anyhow::Error;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use streaker_common::ws::{MemberState, ScanSessionState, StreakState, WsRequest, WsResponse};

mod app;
mod components;
mod qrcode;
mod route;
mod services;
mod util;

use components::Flash;
use components::Footer;
use components::Header;

use services::api;
use services::token;

use route::{
    dashboard::DashBoard, index::Index, login::Login, scan::Scan, scan_complete::ScanComplete,
    AppRoute,
};

use crate::util::if_auth;

#[wasm_bindgen(start)]
pub fn run_app() {
    // initialises logging, needs the wasm_logger crate
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(debug_assertions)]
    log::warn!("Running in Development Mode");

    App::<app::App>::new().mount_to_body();
}
