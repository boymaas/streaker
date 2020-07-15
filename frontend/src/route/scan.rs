use chrono::Utc;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use streaker_common::ws::{MemberState, ScanSessionState, StreakState};

use crate::qrcode;
use crate::route::AppRoute;
use crate::token;
use crate::util::RawHTML;

pub struct Scan {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub member_state: Option<MemberState>,
    pub streak_state: Option<StreakState>,
    pub scan_session_state: Option<ScanSessionState>,
}

impl Component for Scan {
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

        let suuid = &token::get_token_suuid().unwrap().to_string();

        // we can do the unwrap, as the parent component, will not render
        // this component when next_anode is None. This means the scan session
        // has been completed
        let next_anode = scan_session_s.next_anode.as_ref().unwrap();

        // This is in production
        let qrcode = qrcode::generate("Streaker Scan", &next_anode.url, &format!("scan:{}", suuid));

        // NOTE: this is only for development, we override it here
        #[cfg(debug_assertions)]
        let qrcode = qrcode::generate(
            "Streaker Scan",
            "https://opesdentist.monetashi.io",
            &format!("scantest:{}", suuid),
        );

        html! {
        <div id="scan">
            <div class="earned">
              <span class="amount">
                <span>{ "$" }</span>
                 { format!("{:.4}", streak_s.mining_ratio * scan_session_s.count as f64)  }
              </span>
              <span class="subtext">{ "EARNED TODAY" }</span>
            </div>
            <div class="qrcode">
                <RawHTML inner_html={qrcode} />
            </div>

            <div class="stats grid halves">
                <div class="col scansleft">
                  <span class="amount">
                     { scan_session_s.total - scan_session_s.count }
                  </span>
                  <span class="subtext">{ "SCANS LEFT" }</span>
                </div>

                <div class="col remaining">
                  <span class="amount">
                    <span>{ "$" }</span>
                     { format!("{:.4}", streak_s.mining_ratio * ( scan_session_s.total - scan_session_s.count) as f64)  }
                  </span>
                  <span class="subtext">{ "REMAINING TODAY" }</span>
                </div>
            </div>

        </div>

        }
    }
}
