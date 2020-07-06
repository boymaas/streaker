use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::token;
use crate::util::if_auth;

pub struct Menu {}

pub enum Msg {}

impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <>
                <div id="menu" class={ format!("grid thirds {}", if_auth("auth", "")) }>
                <ul>
                   <li>{"DASHBOARD"}</li>
                   <li>{"CASHOUT"}</li>
                   <li>{"SCANS"}</li>
                </ul>

                </div>
            </>
        }
    }
}
