use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct DashBoard {
    link: ComponentLink<Self>,
}

impl Component for DashBoard {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
        <>

            <h1>{ "DashBoard" }</h1>


        </>

        }
    }
}
