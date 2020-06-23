use maud::{html, DOCTYPE};

pub fn head(page_title: Option<&str>) -> maud::Markup {
    let title = match page_title {
        Some(title) => title,
        None => "OPES Unite Streaker",
    };
    html! {
        (DOCTYPE)
            head {
                meta charset="utf-8";
                link rel="stylesheet" type="text/css" href="/static/css/style.css";
                link href="https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,300;0,400;0,600;0,700;0,800;1,300;1,400;1,600;1,700;1,800&display=swap" rel="stylesheet";
                title { (title) }
            }

    }
}

pub fn header() -> maud::Markup {
    html! {
            div id="header" class="grid thirds" {
                div class="col" id="logo" {
                    a href="/" {
                        h1 { span { "OPES Unite Streak Program" } }
                    }
                }
                div class="bigger filler" {
                }
                div class="col conversion-rate ones" {
                    span { "0.025" }
                    span { span { "USD" } span { "/UBUCK" }}
                }

                div id="mobile-with-opes-screenshot" {
                }
            }
    }
}

pub fn streak_program() -> maud::Markup {
    html! {
        div class="streak-program" {
            h1 { span { "STREAK" } " " span {"PROGRAM"} }
            p { "3000 of 10000 spots remaining" }
        }
    }
}

pub fn footer() -> maud::Markup {
    html! {
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
