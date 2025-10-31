use std::fmt::Debug;

use leptos::prelude::*;

use derive_more::{Deref, Display, Into};
use thiserror::Error;

use crate::{components::Checkbox, FieldError, Form, FormError, Name};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Display, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Accept(bool);

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, Hash)]
pub enum AcceptError {
    #[error("This is required")]
    NotAccepted,
}

impl TryFrom<bool> for Accept {
    type Error = AcceptError;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        if value {
            Ok(Accept(true))
        } else {
            Err(AcceptError::NotAccepted)
        }
    }
}

impl Form for Accept {
    fn view(
        label: &'static str,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <Checkbox<Accept> label=label name=name value=value callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                callback.run(v.map_err(FormError::from));
            }))} />
        }
    }
}
