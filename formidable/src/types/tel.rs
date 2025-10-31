use std::{fmt::Debug, str::FromStr};

use derive_more::{Deref, Display, Into};
use thiserror::Error;

use crate::{components::InputType, types::FormType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tel(String);

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, Hash)]
pub enum TelError {
    #[error("Invalid format")]
    InvalidFormat,
}

impl FromStr for Tel {
    type Err = TelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars()
            .all(|c| c.is_ascii_digit() || c == '+' || c == '-' || c == ' ' || c == '(' || c == ')')
            && !s.is_empty()
        {
            Ok(Tel(s.to_string()))
        } else {
            Err(TelError::InvalidFormat)
        }
    }
}

impl FormType for Tel {
    const INPUT_TYPE: InputType = InputType::Tel;
    const REQUIRED: Option<bool> = Some(true);
}
