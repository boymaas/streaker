#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod partials;
mod route;

struct Root {
    link: ComponentLink<Self>,
}

impl Component for Root {
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
        <div class="container" id="index">
           <div class="content">

              { partials::header() }


            <route::index::Index />
            // <route::login::Login />


           </div>
        </div>

        { partials::footer() }


        </>

        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
