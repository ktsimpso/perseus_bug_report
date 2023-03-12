use perseus::prelude::{Html, PerseusApp};
use sycamore::prelude::view;

mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .global_state_creator(crate::templates::global::get_global_state_creator())
        .locales_and_translations_manager("en-US", &["fr-FR"])
        .index_view(|cx| {
            view! {cx,
                head {
                    meta (charset="utf-8") { }
                    meta (name="viewport", content="width=device-width,initial-scale=1") { }
                    link( rel="stylesheet", href="/.perseus/static/css/styles.css") { }
                }
                body {
                    div (id="root") {}
                    span { "Load the local host root page (not the translated pages). To see the error message in the js console after it redirects you to the locale specific page. If you refresh on the translated pages things work as expected." }
                }
            }
        })
}
