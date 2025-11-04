use crate::{
    components::{Input, InputType},
    FieldError, Form, FormError, Name,
};
use derive_more::{Deref, Into};
use leptos::prelude::*;
use std::fmt::Display;
use std::str::FromStr;
use time::macros::format_description;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Date(time::Date);

const DATE_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    format_description!("[year]-[month]-[day]");

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(DATE_FORMAT).unwrap())
    }
}

impl FromStr for Date {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = time::Date::parse(s, DATE_FORMAT)?;
        Ok(Date(date))
    }
}

impl Default for Date {
    fn default() -> Self {
        let now: time::OffsetDateTime =
            time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc());
        Date(now.date())
    }
}

impl Form for Date {
    fn view(
        field: crate::FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <Input<Date>
                label=field.label.expect("No label provided")
                description=field.description
                name=name
                value=value
                callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                    callback.run(v.map_err(FormError::from));
                }))}
                input_type=InputType::Date
            />
        }
    }
}

const DATETIME_LOCAL_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]");

#[derive(Debug, Clone, PartialEq, Eq, Hash, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrimitiveDateTime(time::PrimitiveDateTime);

impl Display for PrimitiveDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(DATETIME_LOCAL_FORMAT).unwrap())
    }
}

impl FromStr for PrimitiveDateTime {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let datetime = time::PrimitiveDateTime::parse(s, DATETIME_LOCAL_FORMAT)?;
        Ok(PrimitiveDateTime(datetime))
    }
}

impl Default for PrimitiveDateTime {
    fn default() -> Self {
        let now: time::OffsetDateTime =
            time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc());
        PrimitiveDateTime(time::PrimitiveDateTime::new(now.date(), now.time()))
    }
}

impl Form for PrimitiveDateTime {
    fn view(
        field: crate::FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <Input<PrimitiveDateTime>
                label=field.label.expect("No label provided")
                description=field.description
                name=name
                value=value
                callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                    callback.run(v.map_err(FormError::from));
                }))}
                input_type=InputType::DatetimeLocal
            />
        }
    }
}

const TIME_FORMAT: &[time::format_description::BorrowedFormatItem<'_>] =
    format_description!("[hour]:[minute]");

#[derive(Debug, Clone, PartialEq, Eq, Hash, Into, Deref)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Time(time::Time);

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(TIME_FORMAT).unwrap())
    }
}

impl FromStr for Time {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time = time::Time::parse(s, TIME_FORMAT)?;
        Ok(Time(time))
    }
}

impl Default for Time {
    fn default() -> Self {
        let now: time::OffsetDateTime =
            time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc());
        Time(now.time())
    }
}

impl Form for Time {
    fn view(
        field: crate::FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <Input<Time>
                label=field.label.expect("No label provided")
                description=field.description
                name=name
                value=value
                callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                    callback.run(v.map_err(FormError::from));
                }))}
                input_type=InputType::Time
            />
        }
    }
}
