use maud::html;

use qrcode_generator::QrCodeEcc;

use crate::web::page::components;

pub fn page() -> maud::Markup {
    // Login string
    let url = "https://mobile.opes.pe/opesapp/check-in?name=OpesUnite&url=https%3A%2F%2Faccess-node.opesunite.io&source=$account_id";

    let result: String =
        qrcode_generator::to_svg_to_string(url, QrCodeEcc::Low, 400, None).unwrap();

    html! {
        (components::head(None))
            body id="login" {
                div class="container" {
                    (components::header());

                    div class="content" {
                        h2 { "Scan with OPES ID app to Join" }

                        p { "Install the App on your device, create an account on the OPES "
                            "Network. Now you can scan this QrCode to join the "
                            "OPES Unite Streak Program."
                        }

                        div class="qrcode" {
                            (maud::PreEscaped(result))
                        }
                        div class="download-app-buttons grid halves" {
                            div.app-store.col {
                                img src="/static/img/app-store-badge.svg";
                            }
                            .google-play.col {
                                img src="/static/img/google-play-badge.svg";
                            }
                        }

                    }

                }

                (components::footer());
            }
    }
}
