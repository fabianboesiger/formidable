use leptos::prelude::*;

use crate::{FieldError, Name};
use std::fmt::Display;
use std::str::FromStr;

#[component]
pub fn Input<T>(
    #[prop(into)] label: &'static str,
    #[prop(into)] name: Name,
    #[prop(into)] value: Option<T>,
    #[prop(into)] callback: Option<Callback<Result<T, FieldError>>>,
    #[prop(into, default = InputType::default())] input_type: InputType,
    #[prop(into, default = None)] min: Option<T>,
    #[prop(into, default = None)] max: Option<T>,
    #[prop(into, default = None)] step: Option<T>,
    #[prop(into, default = None)] placeholder: Option<T>,
    #[prop(into, default = None)] required: Option<bool>,
    #[prop(into, default = None)] minlength: Option<usize>,
    #[prop(into, default = None)] maxlength: Option<usize>,
    #[prop(into, default = Vec::default())] datalist: Vec<T>,
) -> impl IntoView
where
    T: Clone + Display + FromStr + Send + Sync + 'static,
    T::Err: Clone + Display + Send + Sync + 'static,
{
    let node_ref = NodeRef::new();
    let touched = RwSignal::new(false);
    let raw_value = RwSignal::new(value.map(|v| v.to_string()).unwrap_or_default());
    let value = Signal::derive(move || raw_value.get().parse::<T>());

    if let Some(callback) = callback {
        Effect::new(move |_| {
            callback.run(value.get().map_err(|err| FieldError::new(name, err)));
        });
    }

    node_ref.on_load(move |elem: leptos::web_sys::HtmlInputElement| {
        let input_value = elem.value();
        if input_value != raw_value.get_untracked() {
            raw_value.set(input_value);
        }
    });

    view! {
        <div class:error={move || touched.get() && value.get().is_err()} class="field input-field">
            <label for=name.to_string()>{label}</label>
            <input
                node_ref=node_ref
                type={match input_type {
                    InputType::Text => "text",
                    InputType::Email => "email",
                    InputType::Password => "password",
                    InputType::Color => "color",
                    InputType::Date => "date",
                    InputType::Time => "time",
                    InputType::DatetimeLocal => "datetime-local",
                    InputType::Number => "number",
                    InputType::Tel => "tel",
                    InputType::Url => "url",
                    InputType::Range => "range",
                }}
                name=name.to_string()
                id=name.to_string()
                value={move || raw_value.get()}
                colorpick-eyedropper-active={
                    match input_type {
                        InputType::Color => Some("true"),
                        _ => None,
                    }
                }
                on:focus=move |_| {
                    touched.set(true);
                }
                on:input=move |ev| {
                    touched.set(true);
                    let input = event_target_value(&ev);
                    raw_value.set(input.clone());
                }
                min={min.as_ref().map(|v| v.to_string())}
                max={max.as_ref().map(|v| v.to_string())}
                step={step.as_ref().map(|v| v.to_string())}
                placeholder={placeholder.as_ref().map(|v| v.to_string())}
                required={required}
                minlength={minlength.as_ref().copied()}
                maxlength={maxlength.as_ref().copied()}
                list={if !datalist.is_empty() {
                    Some(format!("{}-datalist", name))
                } else {
                    None
                }}
            />
            {
                if !datalist.is_empty() {
                    let list_id = format!("{}-datalist", name);
                    view! {
                        <datalist id=list_id.clone()>
                            { datalist.iter().cloned().map(|item| {
                                view! {
                                    <option value={item.to_string()} />
                                }
                            }).collect::<Vec<_>>() }
                        </datalist>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }
            { move || {
                touched.get().then(move || value.get().err().map(|e| {
                    view! { <span class="error-message">{format!("{}", e)}</span> }
                }))
            }}
        </div>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Color,
    Date,
    Time,
    DatetimeLocal,
    Number,
    Tel,
    Url,
    Range,
}
