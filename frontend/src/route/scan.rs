use yew::prelude::*;
use yew_router::prelude::*;

use streaker_common::ws::MemberState;

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
        let suuid = &token::get_token_suuid().unwrap().to_string();
        let qrcode = qrcode::generate("opesdentist", &format!("scan:{}", suuid));

        html! {
        <div id="scan">
            <div class="qrcode">
                <RawHTML inner_html={qrcode} />
            </div>
        </div>

        }
    }
}
