use anyhow::Error;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use streaker_common::ws::{MemberState, ScanSessionState, StreakState, WsRequest, WsResponse};

use crate::components::Flash;
use crate::components::Footer;
use crate::components::Header;

use crate::services::api;
use crate::services::token;

use crate::config;

use crate::route::{
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

pub struct App {
    current_route: Option<AppRoute>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    api: api::Api,
    fetch_task: Option<FetchTask>,

    flash_message: Option<String>,

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
    ClearFlash,

    SkipCurrentScan,
}

impl Component for App {
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
            api: api::Api::new(),
            fetch_task: None,
            ws_service: WebSocketService::new(),
            ws: None,
            member_state: None,
            scan_session_state: None,
            streak_state: None,

            flash_message: None,
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

                    self.flash_message("Welcome back visitor");
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
            Msg::ClearFlash => {
                self.flash_message = None;
            }
            Msg::SkipCurrentScan => {
                // send a request over our websocket that we want
                // to skip this scan. We also want an updated state
                // over the websocket
                if let Some(next_anode) = self
                    .scan_session_state
                    .as_ref()
                    .and_then(|s| s.next_anode.as_ref())
                {
                    let anode = next_anode.label.clone();
                    self.ws_send(Json(&WsRequest::SkipCurrentScan(anode)));
                } else {
                    self.flash_message("Not possible to skip scan");
                }
            }
            Msg::Route(route) => self.current_route = AppRoute::switch(route),
            Msg::Token(jwt_token) => {
                // we received a new jwt token, lets set it
                // and (re)connect our websocket
                token::set_token(Some(jwt_token.token));
                self.ws_connect(&token::get_token().unwrap());
            }
            Msg::TokenFetchError => {
                // unable to fetch token, this is a real problem, we
                // cannot connect to the websocket if we don't have a valid
                // token. We have to rety getting a token, and inform the
                // user of the problem.
                // TODO: try to refetch
            }
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

                        // tell user we are loggin out
                        self.flash_message("Token expired, logging out");

                        // and now navigate to the unauthenticated
                        // part of the application
                        self.router_agent.send(ChangeRoute(AppRoute::Index.into()));
                    }
                    WsResponse::DoubleConnection(_) => {
                        // TODO: somebody opened another tab with same app
                        // we have to show this is not possible
                        // and disconnect
                        // tell user we are loggin out
                        self.flash_message("Registered a double connect!");

                        // and now navigate to the unauthenticated
                        // part of the application
                        // self.router_agent.send(ChangeRoute(AppRoute::Index.into()));
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
        // get this callback in the scan route, if the user clicks the
        // skip button.
        let on_skip_scan = self.link.callback(|_| Msg::SkipCurrentScan);

        html! {
            <>

            { dev_build_header() }
                <div id="container-background" class={if_auth("auth", "")} ></div>
                <div class="container" id="index">
                <div class={if_auth("auth content", "")}>

                <Header current_route=&self.current_route />

                <Flash message=&self.flash_message duration=Duration::seconds(3) callback=self.link.callback(|_: bool| Msg::ClearFlash) />
                {
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Login => html!{<Login />},
                            AppRoute::Index => html!{<Index  />},
                            AppRoute::DashBoard => html!{<DashBoard member_state=&self.member_state streak_state=&self.streak_state scan_session_state=&self.scan_session_state />},
                            AppRoute::Scans => {
                                if self.scan_session_state.as_ref().map_or(false, |s| s.next_anode.is_some()) {
                                    html!{<Scan member_state=&self.member_state
                                                streak_state=&self.streak_state
                                                scan_session_state=&self.scan_session_state
                                                on_skip_scan=on_skip_scan
                                                />}
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

impl App {
    fn flash_message(&mut self, msg: &str) {
        self.flash_message = Some(msg.into());
    }
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
                &format!("{}/ws/{}", config::WSS_ENDPOINT, token),
                callback,
                notification,
            )
            .unwrap();
        self.ws = Some(task);
    }
    fn ws_send<T: Into<Text>>(&mut self, data: T) {
        if let Some(ws) = &mut self.ws {
            ws.send(data);
        } else {
            self.flash_message("Not connected, try again later");
        }
    }
}
