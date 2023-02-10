use super::global::{content_view, head};
use perseus::template::Template;
use sycamore::prelude::{view, Html, Scope, View};

fn about_page<G: Html>(cx: Scope) -> View<G> {
    let content = view! { cx,
        div {
            "This is the about page"
        }
    };

    content_view(cx, content)
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about").view(about_page).head(head).build()
}
