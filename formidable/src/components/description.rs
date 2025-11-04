use leptos::prelude::*;

#[component]
pub fn Description(#[prop(into, default = None)] description: Option<TextProp>) -> impl IntoView {
    description.map(|desc| {
        view! {
            <p class="description">{desc.get()}</p>
        }
    })
}
