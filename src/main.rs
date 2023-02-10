use perseus::prelude::{Html, PerseusApp};
use sycamore::prelude::view;

mod templates;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .global_state_creator(crate::templates::global::get_global_state_creator())
        .index_view(|cx| {
            view! {cx,
                head {
                    meta (charset="utf-8") { }
                    meta (name="viewport", content="width=device-width,initial-scale=1") { }
                    link( rel="stylesheet", href="/.perseus/static/css/styles.css") { }
                }
                body {
                    div (id="root") {}
                    button (class="toggle") { "Toggle checkbox with javascript" }
                    span { "This button toggles the checkbox from a javascript function. This in theory should also play the animation of the box." }
                    script (src = "/.perseus/static/js/toggle.js") {}
                }
            }
        })
}
