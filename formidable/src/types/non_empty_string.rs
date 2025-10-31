use std::{fmt::Debug, str::FromStr};

use derive_more::{Deref, Display, Into};
use thiserror::Error;

use crate::{components::InputType, types::FormType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NonEmptyString(String);

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, Hash)]
pub enum NonEmptyStringError {
    #[error("Input cannot be empty")]
    IsEmpty,
}

impl FromStr for NonEmptyString {
    type Err = NonEmptyStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(NonEmptyStringError::IsEmpty)
        } else {
            Ok(NonEmptyString(s.to_string()))
        }
    }
}

impl FormType for NonEmptyString {
    const INPUT_TYPE: InputType = InputType::Text;
    const REQUIRED: Option<bool> = Some(true);
}
