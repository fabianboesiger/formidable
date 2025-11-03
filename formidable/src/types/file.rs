use std::fmt::{Debug, Display};

use leptos::prelude::*;

use thiserror::Error;

use crate::{components::FileInput, FieldError, Form, FormError, Name};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct File {
    name: String,
    content_type: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: String, content_type: String, data: Vec<u8>) -> Self {
        File {
            name,
            content_type,
            data,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, Hash)]
pub enum FileError {
    #[error("File too large")]
    FileTooLarge,
    #[error("Invalid file type")]
    InvalidFileType,
    #[error("Unknown file error")]
    UnknownFileError,
    #[error("No file selected")]
    NoFileSelected,
}

impl Form for File {
    fn view(
        field: crate::FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <FileInput
                label=field.label
                description=field.description
                name=name
                value=value
                callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                    callback.run(v.map_err(FormError::from));
                }))}
            />
        }
    }
}
