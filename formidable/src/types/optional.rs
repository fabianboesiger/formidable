use std::{fmt::Debug, fmt::Display, str::FromStr};

use derive_more::{Deref, Into};

use crate::{components::InputType, types::FormType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Optional<T>(Option<T>);

impl<T> From<T> for Optional<T> {
    fn from(value: T) -> Self {
        Optional(Some(value))
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(value: Option<T>) -> Self {
        Optional(value)
    }
}

impl<T> Display for Optional<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, ""),
        }
    }
}

impl<T> FromStr for Optional<T>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Optional(None))
        } else {
            T::from_str(s).map(|value| Optional(Some(value)))
        }
    }
}

impl<T> FormType for Optional<T>
where
    T: FormType,
{
    const INPUT_TYPE: InputType = T::INPUT_TYPE;
    const REQUIRED: Option<bool> = Some(false);
}
