use crate::{
    components::{Input, InputType},
    FieldError, Form, FormError, Name,
};
use derive_more::{Deref, Into};
use leptos::prelude::*;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color(color::Rgba8);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Convert RGB to hex format for HTML color input
        write!(f, "#{:02x}{:02x}{:02x}", self.0.r, self.0.g, self.0.b)
    }
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, Hash)]
pub enum ColorError {
    #[error("Invalid format")]
    InvalidFormat,
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse hex color format (#RRGGBB)
        if !s.starts_with('#') || s.len() != 7 {
            return Err(ColorError::InvalidFormat);
        }

        let hex = &s[1..];
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorError::InvalidFormat)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorError::InvalidFormat)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorError::InvalidFormat)?;

        let rgba8 = color::Rgba8 { r, g, b, a: 255 }; // Full opacity
        Ok(Color(rgba8))
    }
}

impl Default for Color {
    fn default() -> Self {
        // Default to black
        Color(color::Rgba8 {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        })
    }
}

impl Form for Color {
    fn view(
        field: crate::FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <Input<Color>
                label=field.label.expect("No label provided")
                description=field.description
                name=name
                value=value
                callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                    callback.run(v.map_err(FormError::from));
                }))}
                input_type=InputType::Color
            />
        }
    }
}
