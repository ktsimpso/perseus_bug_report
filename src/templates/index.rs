use super::global::{content_view, head};
use perseus::{t, template::Template};
use sycamore::prelude::{view, Html, Scope, View};

fn index_page<G: Html>(cx: Scope) -> View<G> {
    let content = view! { cx,
        div {
            (t!(cx, "index"))
        }
    };

    content_view(cx, content)
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
