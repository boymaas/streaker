use crate::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub fn header() -> Html {
    html! {
      <div id="header" class="grid thirds">
         <div class="col" id="logo">
             <RouterAnchor<AppRoute> route=AppRoute::Index>
               <h1><span>{ "OPES Unite Streak Program" }</span></h1>
             </RouterAnchor<AppRoute>>
         </div>
         <div class="bigger filler"></div>
         <div class="col conversion-rate ones">
             <span>{ "0.025" }</span>
             <span>
                 <span>{ "USD" }</span>
                 <span>{ "/UBUCK" }</span>
             </span>
         </div>
         <div id="mobile-with-opes-screenshot"></div>
      </div>
    }
}

pub fn footer() -> Html {
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
