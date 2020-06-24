use maud::html;

use crate::web::page::components;

pub fn page() -> maud::PreEscaped<String> {
    html! {
        (components::head(None));
        body id="index" {
            div class="container" {
                div class="content" {
                    (components::header());

                    (components::streak_program());

                    h2.earn { "Earn $0.375 a day in 5 minutes" }
                    p.how { "Mine Crypto by Scanning QR-Code(s)" }

                    ul.steps {
                        li { "Install App" };
                        li { "Register" };
                        li { "Build streak" };
                    }

                    div.call-to-action {
                        a.button href="/login" { "START" };
                        br;
                        a.returning-user href="#" { "returning user login here" };
                    }

                    div.instant-cashout {
                        h2 { "Instant cashout in ETH " }
                    }
                }
            }
            (components::footer())
        }
    }
}
