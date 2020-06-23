use maud::html;

use crate::web::page::components::{footer, header};

pub fn page() -> maud::PreEscaped<String> {
    html! {
        (header(None))
        body id="index" {
            div class="container" {
                div id="header" class="grid thirds" {
                    div class="col" id="logo" {
                        h1 { span { "OPES Unite Streak Program" } }
                    }
                    div class="bigger filler" {
                    }
                    div class="col conversion-rate ones" {
                        span { "0.025" }
                        span { span { "USD" } span { "/UBUCK" }}
                    }
                }

                div class="content" {
                    h1.streak-program { span { "STREAK" } " " span {"PROGRAM"} }
                    p.spots-remaining { "3000 of 10000 spots remaining" }

                    h2.earn { "Earn $0.375 a day in 5 minutes" }
                    p.how { "Mine Crypto by Scanning QR-Code(s)" }

                    ul.steps {
                        li { "Install App" };
                        li { "Register" };
                        li { "Build streak" };
                    }

                    div.call-to-action {
                        button { "START" };
                        br;
                        a.returning-user href="#" { "returning user login here" };
                    }


                    div.instant-cashout {
                        h2 { "Instant cashout in ETH " }
                    }

                }

            }

            (footer())
        }
    }
}
