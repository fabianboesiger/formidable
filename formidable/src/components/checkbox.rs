use leptos::prelude::*;

use crate::{FieldError, Name};
use std::fmt::Display;

pub use formidable_derive::Form;

#[component]
pub fn Checkbox<T>(
    #[prop(into)] label: TextProp,
    #[prop(into, default = None)] description: Option<TextProp>,
    #[prop(into)] name: Name,
    #[prop(into)] value: Option<T>,
    #[prop(into)] callback: Option<Callback<Result<T, FieldError>>>,
) -> impl IntoView
where
    T: Clone + Into<bool> + TryFrom<bool> + Send + Sync + 'static,
    T::Error: Clone + Display + Send + Sync + 'static,
{
    let node_ref = NodeRef::new();
    let touched = RwSignal::<bool>::new(false);
    let raw_value = RwSignal::new(value.map(Into::into).unwrap_or_default());
    let value = Signal::derive(move || T::try_from(raw_value.get()));

    if let Some(callback) = callback {
        Effect::new(move |_| {
            callback.run(value.get().map_err(|err| FieldError::new(name, err)));
        });
    }

    node_ref.on_load(move |elem: leptos::web_sys::HtmlInputElement| {
        let input_value = elem.checked();
        if input_value != raw_value.get_untracked() {
            raw_value.set(input_value);
        }
    });

    view! {
        <div class:error={move || touched.get() && value.get().is_err()} class="field checkbox-field">
            <label for=name.to_string()>
                <span class="custom custom-checkbox"></span>
                <input
                    node_ref=node_ref
                    type="checkbox"
                    name=name.to_string()
                    id=name.to_string()
                    checked=move || raw_value.get()
                    on:focus=move |_| {
                        touched.set(true);
                    }
                    on:change=move |ev| {
                        touched.set(true);
                        let checked = event_target_checked(&ev);
                        raw_value.set(checked);
                    }
                />
                {label.get()}
            </label>
            { move || touched.get().then(move || value.get().err().map(|e| view! { <p class="error-message">{format!("{}", e)}</p> })) }
            {
                description.map(|desc| view! {
                    <p class="description">{desc.get()}</p>
                })
            }
        </div>
    }
}
