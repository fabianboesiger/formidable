use leptos::prelude::*;

use formidable::{
    types::{Accept, Color, Date, Email, File, Optional, Tel},
    Form, FormidableServerAction,
};
use leptos::server_fn::codec::Json;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum::{Display, IntoStaticStr, VariantArray};
use url::Url;

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserForm {
    #[form(label = "Personal Information")]
    personal_info: PersonalInfo,
    #[form(label = "Contact Information")]
    contact_info: ContactInfo,
    #[form(label = "Addresses")]
    addresses: Vec<Address>,
    #[form(label = "User Type")]
    user_type: UserType,
    #[form(label = "Account Balance")]
    account_balance: bigdecimal::BigDecimal,
    #[form(label = "Accept Terms")]
    accept_terms: Accept,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct PersonalInfo {
    #[form(label = "Full Name")]
    name: String,
    #[form(label = "Date of Birth")]
    date_of_birth: Date,
    #[form(label = "Eye Color")]
    eye_color: Color,
    #[form(label = "ID Document")]
    id: File,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ContactInfo {
    #[form(label = "Email Address")]
    email: Email,
    #[form(label = "Phone Number")]
    phone: Tel,
    #[form(label = "Website")]
    website: Optional<Url>,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Address {
    #[form(label = "Street")]
    street: String,
    #[form(label = "City")]
    city: String,
    #[form(label = "State")]
    state: String,
    #[form(label = "Zip Code")]
    zip: String,
    #[form(label = "Country")]
    country: Country,
}

#[derive(
    Form,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Default,
    VariantArray,
    IntoStaticStr,
    Display,
    Serialize,
    Deserialize,
)]
enum UserType {
    #[strum(to_string = "Admin")]
    Admin,
    #[strum(to_string = "Regular User")]
    #[default]
    Regular,
}

#[derive(
    Form,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Default,
    VariantArray,
    IntoStaticStr,
    Display,
    Serialize,
    Deserialize,
)]
enum Country {
    #[strum(to_string = "Switzerland")]
    #[default]
    Switzerland,
    #[strum(to_string = "Germany")]
    Germany,
    #[strum(to_string = "France")]
    France,
    #[strum(to_string = "Italy")]
    Italy,
    #[strum(to_string = "Spain")]
    Spain,
    #[strum(to_string = "Portugal")]
    Portugal,
}

#[component]
pub fn ExampleForm() -> impl IntoView {
    view! {
        <FormidableServerAction<HandleUserForm, UserForm> label="Example Form" name="user_form" />
    }
}

#[server(
  endpoint = "handle_user_form",
  input = Json,
  output = Json
)]
async fn handle_user_form(user_form: UserForm) -> Result<(), ServerFnError> {
    leptos::logging::log!("Received form submission: {:?}", user_form);
    Ok(())
}
