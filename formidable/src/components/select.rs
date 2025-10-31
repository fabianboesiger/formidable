use leptos::prelude::*;

use crate::Name;
use std::fmt::Display;
use strum::VariantArray;

#[component]
pub fn Select<T>(#[prop(into)] label: &'static str, name: Name, value: RwSignal<T>) -> impl IntoView
where
    T: Clone
        + Copy
        + Display
        + Into<&'static str>
        + VariantArray
        + PartialEq
        + Send
        + Sync
        + 'static,
{
    view! {
        <div class="field select-field">
            <label for=name.to_string()>{label}</label>
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
                { T::VARIANTS.iter().map(move |&option| {
                    let option_value: &'static str = option.into();
                    let is_selected = move || value.get() == option;
                    view! {
                        <option
                            value=option_value
                            selected=is_selected
                        >
                            {format!("{}", option)}
                        </option>
                    }
                }).collect::<Vec<_>>() }
            </select>
        </div>
    }
}
