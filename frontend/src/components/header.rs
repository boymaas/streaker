use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::token;
use crate::util::{if_auth_borrow, if_auth_move};

use crate::components::Menu;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
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
                <div id="header" class={ format!("grid thirds {}", if_auth_move("auth", "")) }>
                    <div class="col" id="logo">
                        <RouterAnchor<AppRoute> route={ if_auth_move( AppRoute::DashBoard, AppRoute::Index ) }>
                            <h1><span>{ "OPES Unite Streak Program" }</span></h1>
                        </RouterAnchor<AppRoute>>
                    </div>
                    <div class="bigger filler"></div>
                    <div class="col conversion-rate ones">
                        <span>{ "0.035" }</span>
                        <span>
                            <span>{ "USD" }</span>
                            <span>{ "/UBUCK" }</span>
                        </span>
                    </div>
                    <div id="mobile-with-opes-screenshot"></div>

                    {
                        if_auth_move(html! {<Menu />}, html! {})
                    }

                </div>
            </>
        }
    }
}
