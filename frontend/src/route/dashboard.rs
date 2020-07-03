use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

use streaker_common::ws::MemberState;

pub struct DashBoard {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub member_state: Option<MemberState>,
}

impl Component for DashBoard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
        <div id="dashboard">
            <div class="gauges grid halves">
                <div class="streak col">
                   <h2>{ "Streak" }</h2>
                   // { "self.props.member_state.streak_total" }
                   <div class="gauge">{"0"}</div>
                   <div class="unit">{"days"}</div>
                </div>
                <div class="mining-ratio col">
                   <h2>{ "Mining ratio" }</h2>
                   <div class="gauge">{ "40" }</div>
                   <div class="unit"><span>{"UBUCKS"}</span><span>{"/100 SCANs"}</span></div>
                </div>
            </div>

            <p>{ "NEXT LEVEL UP AT 90 DAYS STREAK 50 UB/100" }</p>

            <div class="start-scanning">
                <a class="button">{ "START" }</a>
            </div>

            // <p>{ format!("{:#?}", self.props) }</p>
        </div>

        }
    }
}
