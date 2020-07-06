use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::AppRoute;
use crate::util::if_auth;

pub struct Menu {}

pub enum Msg {}

impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let menu_item = |class: &str, label: &str, route: AppRoute| {
            html! {
                <li class={ class }>
                    <RouterAnchor<AppRoute> route={ route }>
                        <span>{ label }</span>
                    </RouterAnchor<AppRoute>>
                </li>
            }
        };

        html! {
            <>
                <div id="menu">
                <ul>

                   { menu_item("dashboard", "DASHBOARD", AppRoute::DashBoard) }

                   { menu_item("cashouts", "CASHOUTS", AppRoute::CashOuts) }

                   { menu_item("scans", "SCANS", AppRoute::Scans) }

                </ul>

                </div>
            </>
        }
    }
}
