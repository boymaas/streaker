use crate::util::RawHTML;
use qrcode_generator::QrCodeEcc;
use yew::prelude::*;

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
        let url = "https://mobile.opes.pe/opesapp/check-in?name=OpesUnite&url=https%3A%2F%2Fopesdentist.monetashi.io&source=ANID";
        let result: String =
            qrcode_generator::to_svg_to_string(url, QrCodeEcc::Low, 400, None).unwrap();

        html! {

            <div class="content" id="login">
                <h2>{ "Scan with OPES ID app to Join" }</h2>

                <p>{ "Install the App on your device, create an account on the OPES Network. Now you can scan this QrCode to join the OPES Unite Streak Program." }</p>

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
