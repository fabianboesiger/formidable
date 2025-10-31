use leptos::prelude::*;

use crate::Name;
use std::fmt::Display;
use strum::VariantArray;

#[component]
pub fn Radio<T>(#[prop(into)] label: &'static str, name: Name, value: RwSignal<T>) -> impl IntoView
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
        <div>
            <fieldset class="radio-group">
                <legend>{label}</legend>
                { T::VARIANTS.iter().map(move |&option| {
                    let is_checked = move || value.get() == option;
                    let option_value: &'static str = option.into();
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
            </fieldset>
        </div>
    }
}
