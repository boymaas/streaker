use maud::html;

pub fn page() -> maud::PreEscaped<String> {
    html! {
        head {
            link rel="stylesheet" type="text/css" href="/static/css/style.css";
            link href="https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,300;0,400;0,600;0,700;0,800;1,300;1,400;1,600;1,700;1,800&display=swap" rel="stylesheet";
        }
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

            div id="footer"  {
                div class="container grid thirds" {
                    div class="contact col" {
                        h3 { span {} "Contact" }
                        ul {
                            li {
                                a href="contact@qrcodes.io" { "contact@qrcodes.io" }
                            }
                        }
                    }
                    div class="col" {
                    }
                    div class="information col" {
                        h3 { span {} "Information" }

                        ul {
                            li {
                                a href="/about-us" { "About us" }
                            }

                            li {
                                a href="/end-user-agreement" { "End User Agreement" }
                            }
                        }
                    }
                }
            }
            div id="copyright" {
                "Copyright 2020 - OPES Unite"
            }
        }
    }
}
