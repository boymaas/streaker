use maud::html;

pub fn page() -> maud::PreEscaped<String> {
    html! {
        head {
            link rel="stylesheet" type="text/css" href="/static/css/style.css";
        }
        h1 { "Hello world" }
    }
}
