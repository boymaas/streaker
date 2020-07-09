use yew::prelude::*;

use crate::qrcode;
use crate::token;
use crate::util::RawHTML;

pub struct Login {
    link: ComponentLink<Self>,
}

impl Component for Login {
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
        let suuid = &token::get_token_suuid().unwrap().to_string();
        let result = qrcode::generate("Streaker Login", "opesdentist", &format!("login:{}", suuid));

        html! {

            <div class="content" id="login">
                <h2>{ "Scan to start earning daily rewards" }</h2>

                <p>{ "" }</p>

                <div class="qrcode">
                    <RawHTML inner_html={result} />
                </div>
                <div class="download-app-buttons grid halves">
                    <div class="app-store col">
                        <a href="https://apps.apple.com/us/app/opes-id/id1462956865" target="_install_app">
                            <img src="/img/app-store-badge.svg" />
                        </a>
                    </div>
                    <div class="google-play col">
                        <a href="https://play.google.com/store/apps/details?id=one.opes.mobile.opesapp" target="_install_app">
                            <img src="/img/google-play-badge.svg" />
                        </a>
                    </div>
                </div>
            </div>


        }
    }
}
