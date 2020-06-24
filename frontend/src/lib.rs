#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod partials;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
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

              <div class="streak-program">
                 <h1><span>{ "STREAK" }</span> <span>{ "PROGRAM" }</span></h1>
                 <p>{ "3000 of 10000 spots remaining" }</p>
              </div>
              <h2 class="earn">{ "Earn $0.375 a day in 5 minutes" }</h2>
              <p class="how">{ "Mine Crypto by Scanning QR-Code(s)" }</p>
              <ul class="steps">
                 <li>{ "Install App" }</li>
                 <li>{ "Register" }</li>
                 <li>{ "Build streak" }</li>
              </ul>
              <div class="call-to-action">
                 <a class="button" href="/login">{ "START" }</a><br />
                 <a class="returning-user" href="#">{ "returning user login here" }</a>
              </div>
              <div class="instant-cashout">
                 <h2>{ "Instant cashout in ETH " }</h2>
              </div>
           </div>
        </div>

        { partials::footer() }


        </>

        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
