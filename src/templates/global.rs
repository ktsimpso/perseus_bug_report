use perseus::{engine_only_fn, reactor::Reactor, state::GlobalStateCreator, ReactiveState};
use serde::{Deserialize, Serialize};
use sycamore::{
    prelude::{view, Html, Scope, View},
    rt::Event,
};

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
pub struct PageState {
    pub checkbox_state: bool,
}

fn nav<G: Html>(cx: Scope) -> View<G> {
    let state = Reactor::<G>::from_cx(cx).get_global_state::<PageStateRx>(cx);

    #[cfg(client)]
    {
        use perseus::router::RouterLoadState;
        use sycamore::reactive::create_effect;
        use sycamore::rt::JsCast;
        use wasm_bindgen::closure::Closure;
        use web_sys::{window, HtmlInputElement};
        let load_state = Reactor::<G>::from_cx(cx).router_state.get_load_state(cx);
        create_effect(cx, || match *load_state.get() {
            RouterLoadState::Loaded { .. } => {
                window().iter().for_each(|w| {
                    // idealy I'd be able to set state.checkbox_state here instead of having to delve into the dom for this. But lifetimes won't allow it.
                    let closure = Closure::once(|| {
                        window()
                            .and_then(|window| window.document())
                            .and_then(|document| {
                                document.query_selector(".checkbox").unwrap_or(Option::None)
                            })
                            .into_iter()
                            .map(|element| element.unchecked_into::<HtmlInputElement>())
                            .for_each(|element| {
                                element.set_checked(false);
                                Event::new("change").iter().for_each(|event| {
                                    element.dispatch_event(&event);
                                });
                            })
                    });

                    w.set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        100,
                    );

                    closure.forget();
                });
            }
            _ => (),
        });
    }

    let derived_state = state
        .checkbox_state
        .map(cx, |checked| if *checked { "box right" } else { "box" });

    view! {cx,
        nav {
            a (href = "") { "index" }
            a (href = "/about") { "about" }
            span { "Changing the page will uncheck the box after 100ms timeout. This in theory would allow the animation to play." }
        }
        input (type = "checkbox", class = "checkbox", bind:checked=state.checkbox_state) {}
        span { "The box should move when the checkbox is checked. Manual checking an unchecking works as expected." }
        div (class = derived_state) {}
    }
}

pub fn content_view<G: Html>(cx: Scope, content: View<G>) -> View<G> {
    view! { cx,
        (nav(cx))
        main {
            (content)
        }
    }
}

#[engine_only_fn]
pub fn head(cx: Scope) -> View<sycamore::web::SsrNode> {
    view! { cx,
        title { "Title" }
    }
}

#[engine_only_fn]
pub async fn get_common_build_state(locale: String) -> PageState {
    PageState {
        checkbox_state: false,
    }
}

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_common_build_state)
}
