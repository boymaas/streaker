use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::AppRoute;

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub current_route: Option<AppRoute>,
}

pub struct Menu {
    props: Props,
}

impl Component for Menu {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        let menu_item = |class: &str, label: &str, route: AppRoute| {
            let mut cl: String = class.to_owned();

            if let Some(current_route) = &self.props.current_route {
                if route == *current_route {
                    cl = format!("{} current", class);
                }
            };
            html! {
                <li class={ cl }>
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
