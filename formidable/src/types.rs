#![allow(unused_imports)]

mod accept;
#[cfg(feature = "bigdecimal")]
mod bigdecimal;
#[cfg(feature = "color")]
mod color;
mod email;
#[cfg(feature = "file")]
mod file;
mod non_empty_string;
mod optional;
mod std_types;
mod tel;
#[cfg(feature = "time")]
mod time;
#[cfg(feature = "url")]
mod url;

pub use accept::*;
#[cfg(feature = "bigdecimal")]
pub use bigdecimal::*;
#[cfg(feature = "color")]
pub use color::*;
pub use email::*;
#[cfg(feature = "file")]
pub use file::*;
pub use non_empty_string::*;
pub use optional::*;
pub use tel::*;
#[cfg(feature = "time")]
pub use time::*;
#[cfg(feature = "url")]
pub use url::*;

use crate::{
    components::{Input, InputType},
    FieldError, Form, FormError, Name,
};
use leptos::prelude::*;
use std::fmt::Display;
use std::str::FromStr;

impl<T> Form for T
where
    T: FormType,
    <T as FromStr>::Err: Clone + Display + Send + Sync + 'static,
{
    fn view(
        field: crate::FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <Input<T>
                label=field.label.expect("No label provided")
                description=field.description
                name=name
                value=value
                callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                    callback.run(v.map_err(FormError::from));
                }))}
                input_type=T::INPUT_TYPE
                placeholder=T::PLACEHOLDER
                required=T::REQUIRED
                minlength=T::MIN_LENGTH
                maxlength=T::MAX_LENGTH
                min=T::MIN
                max=T::MAX
                step=T::STEP
            />
        }
    }
}

pub trait FormType: Clone + Display + FromStr + Send + Sync + 'static {
    const INPUT_TYPE: InputType;
    const PLACEHOLDER: Option<Self> = None;
    const REQUIRED: Option<bool> = None;
    const MIN_LENGTH: Option<usize> = None;
    const MAX_LENGTH: Option<usize> = None;
    const MIN: Option<Self> = None;
    const MAX: Option<Self> = None;
    const STEP: Option<Self> = None;
}
