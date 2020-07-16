use chrono::Utc;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use streaker_common::ws::{MemberState, ScanSessionState, StreakState};

use crate::components::Clock;
use crate::route::AppRoute;

pub struct ScanComplete {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub member_state: Option<MemberState>,
    pub streak_state: Option<StreakState>,
    pub scan_session_state: Option<ScanSessionState>,
}

impl Component for ScanComplete {
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
        let scan_session_d = ScanSessionState::default();
        let scan_session_s = match &self.props.scan_session_state {
            Some(sss) => sss,
            None => &scan_session_d,
        };

        let streak_d = StreakState::default();
        let streak_s = match &self.props.streak_state {
            Some(ss) => ss,
            None => &streak_d,
        };

        html! {
        <div id="scan" class="completed">
            <div class="earned">
              <span class="amount">
                <span>{ "$" }</span>
                 { format!("{:.4}", streak_s.mining_ratio * scan_session_s.count as f64)  }
              </span>
              <span class="subtext">{ "EARNED TODAY" }</span>
            </div>

            <h2 class="completed-all">
                {"Completed All Scans For Today!"}
            </h2>

            <div class="funny-image">
                <img src="https://media.giphy.com/media/3o85xtib0RaWzZ7U1G/giphy.gif" />
            </div>

            <div class="time-to-next-scan-session">
                <h3>{"NEXT SCANSESSION"}</h3>
                <Clock target_time=scan_session_s.end() />
            </div>
        </div>

        }
    }
}
