use leptos::prelude::*;

use crate::name::Name;

#[component]
pub fn Section(
    #[prop(into)] name: Name,
    #[prop(into, default = None)] heading: Option<TextProp>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="form-section">
            { heading.map(|heading| {
                match name.len() {
                    1 => view! { <h1>{heading.get()}</h1> }.into_any(),
                    2 => view! { <h2>{heading.get()}</h2> }.into_any(),
                    3 => view! { <h3>{heading.get()}</h3> }.into_any(),
                    4 => view! { <h4>{heading.get()}</h4> }.into_any(),
                    5 => view! { <h5>{heading.get()}</h5> }.into_any(),
                    _ => view! { <h6>{heading.get()}</h6> }.into_any(),
                }
            })}
            { children() }
        </div>
    }
}
