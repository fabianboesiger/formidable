use std::fmt::Display;

use leptos::prelude::*;

#[component]
pub fn ErrorMessage<T, E>(
    #[prop(into)] touched: Signal<bool>,
    #[prop(into)] value: Signal<Result<T, E>>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    E: Clone + Display + Send + Sync + 'static,
{
    view! {
        { move || {
            touched.get().then(move || value.get().err().map(|e| {
                view! { <p class="message error-message">{format!("{}", e)}</p> }
            }))
        }}
    }
}
