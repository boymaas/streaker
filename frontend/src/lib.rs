#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod components;
mod qrcode;
mod route;
mod services;
mod util;

#[wasm_bindgen(start)]
pub fn run_app() {
    // initialises logging, needs the wasm_logger crate
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(debug_assertions)]
    log::warn!("Running in Development Mode");

    App::<app::App>::new().mount_to_body();
}
