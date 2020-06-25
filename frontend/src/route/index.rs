use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Index {
    link: ComponentLink<Self>,
}

impl Component for Index {
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
             <RouterAnchor<AppRoute> route=AppRoute::Login classes="button">
                 {"START"}
             </RouterAnchor<AppRoute>>
             <br/>
             <a class="returning-user" href="#">{ "returning user login here" }</a>
          </div>
          <div class="instant-cashout">
             <h2>{ "Instant cashout in ETH " }</h2>
          </div>


        </>

        }
    }
}
