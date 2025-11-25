use std::{fmt::Debug, fmt::Display, str::FromStr};

use crate::{
    components::Checkbox, FieldConfiguration, FieldError, Form, FormConfiguration, FormError,
};
use derive_more::{Deref, Into};
use leptos::prelude::*;

use crate::{components::InputType, types::FormType};

impl<T> Form for Option<T>
where
    T: Form + Clone,
{
    fn view(
        field: crate::FieldConfiguration,
        name: crate::Name,
        value: Option<Self>,
        callback: Option<leptos::prelude::Callback<Result<Self, crate::FormError>>>,
    ) -> impl leptos::IntoView {
        let is_selected = RwSignal::new(value.as_ref().flatten().is_some());
        let last_value = RwSignal::new(value.flatten());

        let callback = callback.map(|cb| Callback::new(move |v| {}));

        view! {
            <div
                class="option"
                style={field.colspan.as_ref().map(|cols| format!("grid-column: span {};", cols))}
            >
                // Variant selector
                <div class="option-state">
                    <Checkbox
                        label=field.label.as_ref().cloned().expect("No label provided")
                        name=name.push_key("selected")
                        value=is_selected.get_untracked()
                        callback={Callback::new(move |selected: Result<bool, FieldError>| {
                            is_selected.set(selected.unwrap_or(false));
                        })}/>
                </div>

                // Variant-specific form
                <div class="option-value">
                    {let field = field.clone(); move || {
                        if is_selected.get() {
                            T::view(FieldConfiguration {
                                label: field.label.clone(),
                                description: field.description.clone(),
                                class: field.class.clone(),
                                colspan: None,
                                placeholder: field.placeholder.clone(),
                            }, name, last_value.get(), callback).into_any()
                        } else {
                            ().into_any()
                        }
                    }}
                </div>
            </div>
        }
        .into_any()
    }
}
