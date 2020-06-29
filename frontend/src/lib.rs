#![recursion_limit = "512"]
use anyhow::Error;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew_router::prelude::*;

use streaker_common::ws::{WsRequest, WsResponse};

mod partials;
mod route;
mod services;
mod util;

use services::api;
use services::token;

use route::{index::Index, login::Login, AppRoute};

struct Root {
    current_route: Option<AppRoute>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    api: services::api::Api,
    fetch_task: Option<FetchTask>,

    ws_service: WebSocketService,
    ws: Option<WebSocketTask>,
}

#[derive(Debug)]
pub enum Msg {
    Route(Route),
    Token(api::JwtToken),
    TokenFetchError,
    WsReady(Result<WsResponse, Error>),
    WsAction(WsAction),
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();

        Self {
            link,
            current_route: AppRoute::switch(route),
            router_agent,
            api: services::api::Api::new(),
            fetch_task: None,
            ws_service: WebSocketService::new(),
            ws: None,
        }
    }

    // called when app initiatlises, we use this to
    // get a token from our backend if we do not have one yet.
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if !token::have_token() {
                // This callback can be anything
                let callback =
                    self.link
                        .callback(|result: Result<api::JwtToken, api::ApiError>| {
                            if let Ok(jwt_token) = result {
                                Msg::Token(jwt_token)
                            } else {
                                Msg::TokenFetchError
                            }
                        });
                // The callback is described here. It will perform the
                // fetch and run the callback either containing a valid
                // response or an error. It is the job from this compent
                // to handle both.

                // TODO: clean this up, we need to check if the fetch could be executed
                // and otherwise display some kind of error.
                //
                // NOTE: the fetch task must exist for the duration of the request
                //       on a drop it will abort the request.
                self.fetch_task = Some(self.api.token_fetch(callback).unwrap());
            } else {
                // we have a token, as such check if its authenticated
                // and ifso navigate towards the logged in area
                if token::is_authenticated() {
                    log::info!("Authenticated token")
                }
            }

            // now build up a websocket connection
            self.ws_connect();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => self.current_route = AppRoute::switch(route),
            Msg::Token(jwt_token) => token::set_token(Some(jwt_token.token)),
            Msg::TokenFetchError => {}
            Msg::WsReady(Ok(response)) => {
                log::info!("WsReady {:?}", response);
                match response {
                    WsResponse::Connected => log::info!("Connected"),
                    WsResponse::Authenticated(visitor_id) => {
                        // we are authenticated by a scan, navigate
                        // towards the secure area
                    }
                }
            }

            Msg::WsReady(Err(e)) => {
                log::error!("WsReady::Error {:?}", e);
            }
            Msg::WsAction(action) => {
                log::info!("WsAction {:?}", action);
                match action {
                    WsAction::Lost => log::info!("Lost connection, trying to reconnect"),
                    _ => {}
                }
            }
            _ => log::warn!("Uncaught Msg: {:?}", msg),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="container" id="index">
                <div class="content">

                { partials::header() }
            {
                if let Some(route) = &self.current_route {
                    match route {
                        AppRoute::Login => html!{<Login />},
                        AppRoute::Index => html!{<Index />},
                    }
                } else {
                    // 404 when route matches no component
                    html! { "No child component available" }
                }
            }


                </div>
                </div>

                { partials::footer() }


            </>

        }
    }
}

// Websocket relevant code

// WsAction are messages to be send over the callback
// link. We can react on those to reconnect, when
// connection is lost.
//
// on a WebSocket status of error, WsAction closed is
// called again.
#[derive(Debug)]
pub enum WsAction {
    Connected,
    Lost,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

impl Root {
    fn ws_connect(&mut self) {
        // NOTE: this is interesting, I specify Json(data) in the type
        // signature. The fact that the callback has this type signals
        // to the ws_service what to send
        let callback = self.link.callback(|Json(data)| Msg::WsReady(data));
        let notification = self.link.callback(|status| match status {
            WebSocketStatus::Opened => WsAction::Connected,
            WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
        });
        let task = self
            .ws_service
            .connect("ws://localhost:8080/ws", callback, notification)
            .unwrap();
        self.ws = Some(task);
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    // initialises logging, needs the wasm_logger crate
    wasm_logger::init(wasm_logger::Config::default());
    App::<Root>::new().mount_to_body();
}
