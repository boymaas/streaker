#![recursion_limit = "512"]
use anyhow::Error;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::prelude::*;

mod partials;
mod route;
mod services;
mod util;

use route::{index::Index, login::Login, AppRoute};

struct Root {
    current_route: Option<AppRoute>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    api: services::api::Api,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    Route(Route),
    Token,
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
        }
    }

    // called when app initiatlises, we use this to
    // get a token from our backend if we do not have one yet.
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            log::debug!("First Render");
            // This callback can be anything
            let callback = self.link.callback(|_: Result<String, Error>| Msg::Token);
            // The callback is described here. It will perform the
            // fetch and run the callback either containing a valid
            // response or an error. It is the job from this compent
            // to handle both.

            // TODO: clean this up, we need to check if the fetch could be executed
            // and otherwise display some kind of error.
            self.fetch_task = Some(self.api.token_fetch(callback).unwrap());
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => self.current_route = AppRoute::switch(route),
            Msg::Token => log::debug!("Token received"),
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

#[wasm_bindgen(start)]
pub fn run_app() {
    // initialises logging, needs the wasm_logger crate
    wasm_logger::init(wasm_logger::Config::default());
    App::<Root>::new().mount_to_body();
}
