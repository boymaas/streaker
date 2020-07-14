use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

use streaker_common::rewards_program::RewardsProgram;
use streaker_common::ws::{MemberState, StreakState};

pub struct DashBoard {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub member_state: Option<MemberState>,
    pub streak_state: Option<StreakState>,
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
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // not we need to clone the option, to gget the value
        // out of the option. As the options is behind a shared reference.
        let streak_state = self.props.streak_state.clone().unwrap_or_default();

        // TODO: we have a max level, communicate this
        let nlevel_streaks = RewardsProgram::find_streak_bucket(streak_state.bucket + 1);
        let nlevel_mining_ratio = RewardsProgram::find_mining_ratio(streak_state.bucket + 1);

        html! {
        <div id="dashboard">
            <div class="gauges grid halves">
                <div class="streak col">
                   <h2>{ "Streak" }</h2>
                   // { "self.props.member_state.streak_total" }
                   <div class="gauge">{ streak_state.streak_current }</div>
                   <div class="unit">{"days"}</div>
                </div>
                <div class="mining-ratio col">
                   <h2>{ "Mining ratio" }</h2>
                   <div class="gauge">{ format!("{:.04}", streak_state.mining_ratio * 100.) }</div>
                   <div class="unit"><span>{"UBUCKS"}</span><span>{"/100 SCANs"}</span></div>
                </div>
            </div>

            <p>{ format!("NEXT LEVEL UP AT {} DAYS STREAK {:.04} UB/100", nlevel_streaks, nlevel_mining_ratio * 100.) }</p>

            <div class="start-scanning">
                <a class="button">{ "START" }</a>
            </div>

            // <p>{ format!("{:#?}", self.props) }</p>
        </div>

        }
    }
}
