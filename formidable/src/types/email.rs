use std::{fmt::Debug, str::FromStr};

use derive_more::{Deref, Display, Into};
use thiserror::Error;

use crate::{components::InputType, types::FormType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Email(String);

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, Hash)]
pub enum EmailError {
    #[error("Invalid format")]
    InvalidFormat,
}

impl FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('@') {
            Ok(Email(s.to_string()))
        } else {
            Err(EmailError::InvalidFormat)
        }
    }
}

impl FormType for Email {
    const INPUT_TYPE: InputType = InputType::Email;
    const REQUIRED: Option<bool> = Some(true);
}
