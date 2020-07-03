use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {

        <>

        <div id="footer">
           <div class="container grid thirds">
              <div class="contact col">
                 <h3><span></span>{ "Contact" }</h3>
                 <ul>
                    <li><a href="mailto:contact@qrcodes.io">{ "contact@qrcodes.io" }</a></li>
                 </ul>
              </div>
              <div class="col"></div>
              <div class="information col">
                 <h3><span></span>{ "Information" }</h3>
                 <ul>
                    <li><a href="/about-us">{ "About us" }</a></li>
                    <li><a href="/end-user-agreement">{ "End User Agreement" }</a></li>
                 </ul>
              </div>
           </div>
        </div>

        <div id="copyright">{ "Copyright 2020 - OPES Unite" }</div>

        </>

        }
    }
}
