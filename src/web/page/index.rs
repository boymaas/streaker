use maud::html;

pub fn page() -> maud::PreEscaped<String> {
    html! {
        h1 { "Hello world" }
    }
}
