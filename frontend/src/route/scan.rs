use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

use streaker_common::ws::MemberState;

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
        html! {
        <div id="scan">
            // <p>{ format!("{:#?}", self.props) }</p>
        </div>

        }
    }
}
