#![allow(non_snake_case)]

pub mod components;
mod error;
mod name;
pub mod types;

pub use error::*;
pub use name::*;

pub use formidable_derive::Form;
use std::{fmt::Display, marker::PhantomData};

use leptos::{ev::SubmitEvent, prelude::*, server_fn::ServerFn};

use std::fmt::Debug;



pub trait Form: Sized + Send + Sync + 'static {
    fn view(
        label: &'static str,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView;
}

#[component]
pub fn FormidableCallback<T>(
    #[prop(into)] label: &'static str,
    #[prop(into)] name: Name,
    #[prop(optional)] value: Option<T>,
    #[prop(optional)] callback: Option<Callback<Result<T, FormError>>>,
) -> impl IntoView
where
    T: Form,
{
    T::view(label, name, value, callback)
}

#[component]
pub fn FormidableSignal<T>(
    #[prop(into)] label: &'static str,
    #[prop(into)] name: Name,
    #[prop(into)] value: RwSignal<T>,
) -> impl IntoView
where
    T: Form + Clone,
{
    let callback = Callback::new(move |form_result: Result<T, FormError>| {
        if let Ok(v) = form_result {
            value.set(v);
        }
    });

    T::view(label, name, Some(value.get_untracked()), Some(callback))
}

#[component]
pub fn FormidableServerAction<F, T>(
    #[prop(into)] label: &'static str,
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
            {T::view(label, name, value, Some(form_callback)) }
            <button type="submit" disabled=submit_disabled>"Submit"</button>
            { move ||
                if submit.pending().get() {
                    Some(view! { <span class="info-message">"Submitting ..."</span> }.into_any())
                } else {
                    submit.value().get().map(|res| match res {
                        Ok(_) => view! { <span class="success-message">"Form submitted successfully!"</span> }.into_any(),
                        Err(err) => view! { <span class="error-message">{format!("{}", err)}</span> }.into_any(),
                    })
                }

            }
        </form>
    }
}
