use chrono::Utc;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

use streaker_common::ws::{MemberState, ScanSessionState, StreakState};

use crate::browser_detect;
use crate::qrcode;
use crate::route::AppRoute;
use crate::services::token;
use crate::util::RawHTML;

pub struct Scan {
    props: Props,
    link: ComponentLink<Self>,
    not_working: bool,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub member_state: Option<MemberState>,
    pub streak_state: Option<StreakState>,
    pub scan_session_state: Option<ScanSessionState>,
    // TODO: this introduces coupling to the app crate,
    // maybe the other way around is better. As the app
    // crate already knows.
    pub on_skip_scan: Callback<Msg>,
}

pub enum Msg {
    NotWorking,
    SkipCurrentScan,
}

impl Component for Scan {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            not_working: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NotWorking => {
                self.not_working = true;
                true
            }
            Msg::SkipCurrentScan => {
                self.props.on_skip_scan.emit(Msg::SkipCurrentScan);
                false
            }
        }
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
        log::info!("{:?}", next_anode);

        // This is in production
        let checkin_name = format!("Streaker%20Scan%20{}", &next_anode.label);
        let checkin_url =
            qrcode::generate_url(&checkin_name, &next_anode.url, &format!("scan:{}", suuid));

        // NOTE: this is only for development, we override it here
        #[cfg(debug_assertions)]
        let checkin_url = qrcode::generate_url(
            &checkin_name,
            "https://opesdentist.monetashi.io",
            &format!("scantest@{}:{}", next_anode.label, suuid),
        );

        // not working means this scan could not be performed for some reason,
        // this is a way the user can skip this scan. He will not be rewarded
        // for the scan, but the session will be continued.
        let on_not_working = self.link.callback(|_| Msg::NotWorking);

        let on_skip = self.link.callback(|_| Msg::SkipCurrentScan);

        html! {
        <div id="scan">
            <div class="earned">
              <span class="amount">
                <span>{ "$" }</span>
                 { format!("{:.4}", streak_s.mining_ratio * scan_session_s.count as f64)  }
              </span>
              <span class="subtext">{ "EARNED TODAY" }</span>
            </div>

            {
                if browser_detect::is_mobile() {
                    html! {
                        <a class="checkin-button" href={ checkin_url }>
                            <span class="action">{ "SCAN" }</span>
                            <span class="anode">{ &next_anode.label }</span>
                        </a>
                    }
                }
                else {
                    html! {
                        <div class="qrcode">
                            <RawHTML inner_html={qrcode::generate(&checkin_url)} />
                        </div>
                    }
                }
            }

            <div class="notworking">
            {
                if !self.not_working {
                    html! { <a href="#" onclick=on_not_working>{ "Not working?" }</a> }
                } else {
                    html! {
                        <>
                        <a href="#" class="skip" onclick=on_skip>{ "Skip" }</a>
                        <p>{"Sometimes a scan does not take, no worry, just click Skip and continue earning. Please note that each skip is not rewarded with UBUCKS"}</p>
                        </>
                    }
                }
            }
            </div>


            { if !self.not_working { html! {

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
              }} else { html! {}}}
        </div>

        }
    }
}
