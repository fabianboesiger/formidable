use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::Name;

#[derive(Clone)]
pub enum RawValue {
    String(String),
    Other,
}

#[derive(Clone)]
pub struct FieldError {
    name: Name,
    error: Arc<dyn Display + Send + Sync>,
    raw_value: RawValue,
}

impl Debug for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FieldError {{ name: {}, error: {} }}",
            self.name, self.error
        )
    }
}

impl Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl FieldError {
    pub fn new_string<E>(name: Name, err: E, raw_value: String) -> Self
    where
        E: Display + Send + Sync + 'static,
    {
        FieldError {
            name,
            error: Arc::new(err),
            raw_value: RawValue::String(raw_value),
        }
    }

    pub fn new<E>(name: Name, err: E) -> Self
    where
        E: Display + Send + Sync + 'static,
    {
        FieldError {
            name,
            error: Arc::new(err),
            raw_value: RawValue::Other,
        }
    }

    pub fn inner(&self) -> Arc<dyn Display + Send + Sync> {
        self.error.clone()
    }
}

#[derive(Clone, Debug)]
pub struct FormError {
    errors: Vec<FieldError>,
}

impl Display for FormError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.errors {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl From<Vec<FieldError>> for FormError {
    fn from(errors: Vec<FieldError>) -> Self {
        FormError { errors }
    }
}

impl From<FieldError> for FormError {
    fn from(err: FieldError) -> Self {
        FormError { errors: vec![err] }
    }
}

impl Extend<FieldError> for FormError {
    fn extend<T: IntoIterator<Item = FieldError>>(&mut self, other: T) {
        self.errors.extend(other);
    }
}

impl IntoIterator for FormError {
    type Item = FieldError;
    type IntoIter = std::vec::IntoIter<FieldError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}
