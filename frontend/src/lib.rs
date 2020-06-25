#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod partials;
mod route;

use route::{index::Index, login::Login, AppRoute};

struct Root {
    current_route: Option<AppRoute>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Route(Route),
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => self.current_route = AppRoute::switch(route),
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
    App::<Root>::new().mount_to_body();
}
