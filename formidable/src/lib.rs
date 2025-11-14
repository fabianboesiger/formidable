#![allow(non_snake_case)]

pub mod components;
mod error;
mod name;
pub mod types;

pub use error::*;
pub use name::*;

use derive_more::Display;
pub use formidable_derive::Form;
use std::{fmt::Display, marker::PhantomData, sync::Arc};

use leptos::{ev::SubmitEvent, prelude::*, server_fn::ServerFn};

use std::fmt::Debug;

pub use strum;

pub struct FieldConfiguration {
    pub label: Option<TextProp>,
    pub description: Option<TextProp>,
    pub class: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FormConfiguration {
    pub section_label: SectionHeading,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    pub fn to_usize(&self) -> usize {
        match self {
            HeadingLevel::H1 => 1,
            HeadingLevel::H2 => 2,
            HeadingLevel::H3 => 3,
            HeadingLevel::H4 => 4,
            HeadingLevel::H5 => 5,
            HeadingLevel::H6 => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SectionHeading {
    LeveledHeading(HeadingLevel),
    SameHeading(HeadingLevel),
    #[default]
    PlainText,
}

pub trait Form: Sized + Send + Sync + 'static {
    fn view(
        field: FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView;
}

#[component]
pub fn FormidableCallback<T>(
    #[prop(into)] label: TextProp,
    #[prop(into, optional)] description: Option<TextProp>,
    #[prop(into, optional)] form_configuration: FormConfiguration,
    #[prop(into)] name: Name,
    #[prop(optional)] value: Option<T>,
    #[prop(optional)] callback: Option<Callback<Result<T, FormError>>>,
) -> impl IntoView
where
    T: Form,
{
    provide_context(form_configuration);

    T::view(
        FieldConfiguration {
            label: Some(label),
            description,
            class: None,
        },
        name,
        value,
        callback,
    )
}

#[component]
pub fn FormidableRwSignal<T>(
    #[prop(into)] label: TextProp,
    #[prop(into, optional)] description: Option<TextProp>,
    #[prop(into, optional)] form_configuration: FormConfiguration,
    #[prop(into)] name: Name,
    #[prop(into)] value: RwSignal<T>,
) -> impl IntoView
where
    T: Form + Clone,
{
    provide_context(form_configuration);

    let callback = Callback::new(move |form_result: Result<T, FormError>| {
        if let Ok(v) = form_result {
            value.set(v);
        }
    });

    T::view(
        FieldConfiguration {
            label: Some(label),
            description,
            class: None,
        },
        name,
        Some(value.get_untracked()),
        Some(callback),
    )
}

#[component]
pub fn FormidableServerAction<F, T>(
    #[prop(into)] label: TextProp,
    #[prop(into, optional)] description: Option<TextProp>,
    #[prop(into, optional)] form_configuration: FormConfiguration,
    #[prop(into)] name: Name,
    #[prop(optional)] value: Option<T>,
    #[prop(optional)] callback: Option<Callback<F::Output, F::Error>>,
    #[prop(optional)] _phantom: PhantomData<F>,
) -> impl IntoView
where
    T: Form + Clone + Debug + Send + Sync + 'static,
    F: ServerFn + Clone + From<T> + Send + Sync + 'static,
    F::Output: Clone + Send + Sync + 'static,
    F::Error: Clone + Send + Sync + Display + 'static,
{
    provide_context(form_configuration);

    let submit = ServerAction::<F>::new();
    let curr_value = RwSignal::new(value.as_ref().map(|v| Ok(v.clone())));
    let form_callback = Callback::new(move |form_result: Result<T, FormError>| {
        curr_value.set(Some(form_result));
    });
    let submit_disabled = Signal::derive(move || {
        curr_value
            .get()
            .and_then(|v: Result<T, FormError>| v.ok())
            .is_none()
            || submit.pending().get()
    });

    let on_submit = {
        move |ev: SubmitEvent| {
            if ev.default_prevented() {
                return;
            }

            ev.prevent_default();

            if let Some(value) = curr_value.get_untracked().and_then(|v| v.ok()) {
                submit.dispatch(value.into());
            }
        }
    };

    if let Some(callback) = callback {
        Effect::new(move || {
            if let Some(res) = submit.value().get() {
                callback.run(res.unwrap());
            }
        });
    }

    view! {
        <form on:submit=on_submit>
            {T::view(FieldConfiguration {
                label: Some(label),
                description,
                class: None,
            }, name, value, Some(form_callback)) }
            <button type="submit" disabled=submit_disabled>{t(FormMessage::SubmitButton)}</button>
            { move ||
                if submit.pending().get() {
                    Some(view! { <p class="message info-message">{t(FormMessage::SubmitPendingMessage)}</p> }.into_any())
                } else {
                    submit.value().get().map(|res| match res {
                        Ok(_) => view! { <p class="message success-message">{t(FormMessage::SubmitSuccessMessage)}</p> }.into_any(),
                        Err(_err) => view! { <p class="message error-message">{t(FormMessage::SubmitErrorMessage)}</p> }.into_any(),
                    })
                }

            }
        </form>
    }
}

#[derive(Clone)]
pub struct Translation<T>(Arc<dyn Fn(T) -> String>);

impl<T, F> From<F> for Translation<T>
where
    F: Fn(T) -> String + 'static,
{
    fn from(f: F) -> Self {
        Translation(Arc::new(f))
    }
}

impl<T> Translation<T> {
    pub fn apply(&self, text: T) -> String {
        (self.0)(text)
    }
}

pub(crate) fn t<T: Display + Clone + 'static>(text: T) -> String {
    let translation = use_context::<Translation<T>>();
    if let Some(translation) = translation {
        translation.apply(text)
    } else {
        format!("{}", text)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum FormMessage {
    #[display("Submit")]
    SubmitButton,
    #[display("Add")]
    AddButton,
    #[display("Remove")]
    RemoveButton,
    #[display("Submitting ...")]
    SubmitPendingMessage,
    #[display("Form submitted successfully")]
    SubmitSuccessMessage,
    #[display("Error submitting form")]
    SubmitErrorMessage,
}
