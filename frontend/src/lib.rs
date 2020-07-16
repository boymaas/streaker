#![recursion_limit = "512"]
use anyhow::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use streaker_common::ws::{MemberState, ScanSessionState, StreakState, WsRequest, WsResponse};

mod components;
mod qrcode;
mod route;
mod services;
mod util;

use components::Footer;
use components::Header;

use services::api;
use services::token;

use route::{
    dashboard::DashBoard, index::Index, login::Login, scan::Scan, scan_complete::ScanComplete,
    AppRoute,
};

use crate::util::if_auth;

#[cfg(debug_assertions)]
fn dev_build_header() -> Html {
    html! {
        <div id="development-build">{ "DEVBUILD" }</div>
    }
}

#[cfg(not(debug_assertions))]
fn dev_build_header() -> Html {
    html! {}
}

struct Root {
    current_route: Option<AppRoute>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    api: services::api::Api,
    fetch_task: Option<FetchTask>,

    ws_service: WebSocketService,
    ws: Option<WebSocketTask>,

    member_state: Option<MemberState>,
    scan_session_state: Option<ScanSessionState>,
    streak_state: Option<StreakState>,
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
        // router agent runs on the background listening for route changes
        // and when they occur, will send a Msg::Route over the link
        //
        // the fact you can specify Msg::Route as a function implies an
        // enum is a function
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();

        Self {
            link,
            // the switch is an derive macro on our routes
            // transforming the url route to the enum value
            current_route: AppRoute::switch(route),
            router_agent,
            api: services::api::Api::new(),
            fetch_task: None,
            ws_service: WebSocketService::new(),
            ws: None,
            member_state: None,
            scan_session_state: None,
            streak_state: None,
        }
    }

    // called when app initiatlises, we use this to
    // get a token from our backend if we do not have one yet.
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if !token::have_token() {
                // We do not have a token, lets get one,
                // no token means we are not authenticated
                // yet, as such, we have to register
                // the suid to our websocket connection.
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

                // Since we did not have a token, we are not authorized so go to index
                self.router_agent.send(ChangeRoute(AppRoute::Index.into()));
            } else {
                // we have a token, as such check if its authenticated
                // and ifso navigate towards the logged in area
                if token::is_authenticated() {
                    log::info!("Authenticated token");

                    // TODO: when we are authenticated, only change
                    // route when not in set
                    match self.current_route {
                        Some(AppRoute::Scans) => {}
                        // all other routes redirect
                        _ => self
                            .router_agent
                            .send(ChangeRoute(AppRoute::DashBoard.into())),
                    }
                } else {
                    self.router_agent.send(ChangeRoute(AppRoute::Index.into()));
                }

                // in any case, we can open our websocket connection
                // witht the sid in the token

                // now build up a websocket connection
                // we know we have a token, so we can unwrap

                // we assume the token is valid here, if its not valid
                // we will receive as BadToken response, with a new
                // token, which we will use.
                self.ws_connect(&token::get_token().unwrap());
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => self.current_route = AppRoute::switch(route),
            Msg::Token(jwt_token) => {
                // we received a new jwt token, lets set it
                // and (re)connect our websocket
                token::set_token(Some(jwt_token.token));
                self.ws_connect(&token::get_token().unwrap());
            }
            Msg::TokenFetchError => {}
            Msg::WsReady(Ok(response)) => {
                log::info!("WsReady {:?}", response);
                match response {
                    WsResponse::Connected => log::info!("Connected"),
                    WsResponse::BadToken(token) => {
                        // tried connecting via the websocket and the
                        // token is either expired, or invalid signature
                        // so we change our token and reconnect
                        token::set_token(Some(token.clone()));
                        self.ws_connect(&token);
                    }
                    WsResponse::DoubleConnection => {
                        // somebody opened another tab with same app
                        // we have to show this is not possible
                        // and disconnect
                    }
                    WsResponse::Attribution(authenticated_token) => {
                        log::info!("Received attribution request");
                        // We have successfully scanned the token
                        // and received an attribution request.
                        // Lets set our authenticated token!
                        token::set_token(Some(authenticated_token));
                        // reconnect our websocket so we have a connection
                        // with a visitor id on the backend. Needed
                        // to tie the scans to the logged in member
                        self.ws_connect(&token::get_token().unwrap());

                        // and now navigate to the authenticated
                        // part of the application
                        self.router_agent
                            .send(ChangeRoute(AppRoute::DashBoard.into()));
                    }
                    WsResponse::MemberState(member_state) => {
                        log::info!("{:?}", member_state);
                        self.member_state = Some(member_state);
                    }

                    WsResponse::StreakState(streak_state) => {
                        log::info!("{:?}", streak_state);
                        self.streak_state = Some(streak_state);
                    }

                    WsResponse::ScanSessionState(scan_session_state) => {
                        log::info!("{:?}", scan_session_state);
                        self.scan_session_state = Some(scan_session_state);
                    }
                    WsResponse::Error(msg) => {
                        log::error!("{:?}", msg);
                    }
                }
            }

            Msg::WsReady(Err(e)) => {
                log::error!("WsReady::Error {:?}", e);
            }
            Msg::WsAction(action) => {
                log::info!("WsAction {:?}", action);
                match action {
                    WsAction::Lost => {
                        log::info!("Lost connection, trying to reconnect");
                        // this will happen quickly, the client will manage the throttling
                        self.ws_connect(&token::get_token().unwrap());
                    }
                    ws_action => log::warn!("Unhandled WsAction {:?}", ws_action),
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

            { dev_build_header() }
                <div id="container-background" class={if_auth("auth", "")} ></div>
                <div class="container" id="index">
                <div class="content">

                <Header current_route=&self.current_route />
                {
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Login => html!{<Login />},
                            AppRoute::Index => html!{<Index  />},
                            AppRoute::DashBoard => html!{<DashBoard member_state=&self.member_state streak_state=&self.streak_state scan_session_state=&self.scan_session_state />},
                            AppRoute::Scans => {
                                if self.scan_session_state.as_ref().map_or(false, |s| s.next_anode.is_some()) {
                                    html!{<Scan member_state=&self.member_state streak_state=&self.streak_state scan_session_state=&self.scan_session_state />}
                                } else {
                                    // TODO: display a nice page with completed message
                                    // and a timer displaying when you can scan again.
                                    html!{<ScanComplete member_state=&self.member_state streak_state=&self.streak_state scan_session_state=&self.scan_session_state />}
                                }
                            },
                            _ => html!{<p class="no-impl">{ "Not Implemented Yet" }</p>}

                        }
                    } else {
                        // 404 when route matches no component
                        html! { "No child component available" }
                    }
                }


            </div>
                </div>

                <Footer />


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
    fn ws_connect(&mut self, token: &str) {
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
            .connect(
                &format!("ws://localhost:8080/ws/{}", token),
                callback,
                notification,
            )
            .unwrap();
        self.ws = Some(task);
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    // initialises logging, needs the wasm_logger crate
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(debug_assertions)]
    log::warn!("Running in Development Mode");
    App::<Root>::new().mount_to_body();
}
