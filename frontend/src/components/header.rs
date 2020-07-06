use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::util::if_auth;

use crate::components::Menu;

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub current_route: Option<AppRoute>,
}

pub struct Header {
    props: Props,
}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        html! {
            <>
                <div id="header" class={ format!("grid thirds {}", if_auth("auth", "")) }>
                    <div class="col" id="logo">
                        <RouterAnchor<AppRoute> route={ if_auth( AppRoute::DashBoard, AppRoute::Index ) }>
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

                </div>


                {
                    if_auth(html! {<Menu current_route=&self.props.current_route />}, html! {})
                }
            </>
        }
    }
}
