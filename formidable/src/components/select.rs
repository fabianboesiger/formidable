use std::fmt::Display;

use leptos::prelude::*;

use crate::strum::VariantArray;
use crate::{components::Description, Name};

#[component]
pub fn Select<T>(
    #[prop(into)] label: TextProp,
    #[prop(into, default = None)] description: Option<TextProp>,
    name: Name,
    value: RwSignal<T>,
    #[prop(into, default = None)] class: Option<String>,
    #[prop(into, default = None)] colspan: Option<u32>,
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
        <div
            class={format!("field select-field{}", class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default())}
            style={colspan.map(|cols| format!("grid-column: span {};", cols))}
        >
            <label for=name.to_string()>{label.get()}</label>
            <select
                name=name.to_string()
                id=name.to_string()
                on:change=move |ev| {
                    let selected_value = event_target_value(&ev);
                    // Find the variant that matches the selected value
                    if let Some(&option) = T::VARIANTS.iter().find(|&&variant| {
                        let variant_str: &'static str = variant.into();
                        variant_str == selected_value
                    }) {
                        value.set(option);
                    }
                }
            >
                { <T as VariantArray>::VARIANTS.iter().map(move |&option| {
                    let option_value: &'static str = option.into();
                    let is_selected = move || value.get() == option;
                    //let value_label = value_label(&option);
                    view! {
                        <option
                            value=option_value
                            selected=is_selected
                        >
                            {format!("{}", option)}
                        </option>
                    }
                }).collect::<Vec<_>>() }
                <Description description={description} />
            </select>
        </div>
    }
}
