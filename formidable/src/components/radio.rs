use std::fmt::Display;

use leptos::prelude::*;

use crate::strum::VariantArray;
use crate::{components::Description, Name};

#[component]
pub fn Radio<T>(
    #[prop(into)] label: TextProp,
    #[prop(into, default = None)] description: Option<TextProp>,
    name: Name,
    value: RwSignal<T>,
    #[prop(into, default = None)] class: Option<String>,
    //value_label: impl Fn(&T) -> TextProp + 'static,
) -> impl IntoView
where
    T: Clone
        + Copy
        + Into<&'static str>
        + VariantArray
        + PartialEq
        + Display
        + Send
        + Sync
        + 'static,
{
    view! {
        <div class={format!("field radio-group-field{}", class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default())}>
            <fieldset>
                <legend>{label.get()}</legend>
                { <T as VariantArray>::VARIANTS.iter().map(move |&option| {
                    let is_checked = move || value.get() == option;
                    let option_value: &'static str = option.into();
                    //let value_label = value_label(&option);
                    view! {
                        <div class="field radio-field">
                            <label for=format!("{}-{}", name.to_string(), option_value)>
                                <span class="custom custom-radio-button"></span>
                                <input
                                    type="radio"
                                    name=name.to_string()
                                    id=format!("{}-{}", name.to_string(), option_value)
                                    value=option_value
                                    checked=is_checked
                                    on:change=move |_| {
                                        value.set(option);
                                    }
                                />
                                {format!("{}", option)}
                            </label>
                        </div>
                    }
                }).collect::<Vec<_>>() }
                <Description description={description} />
            </fieldset>
        </div>
    }
}
