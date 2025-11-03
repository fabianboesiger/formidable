use leptos::prelude::*;

use formidable::{
    types::{Accept, Color, Date, Email, File, Optional, Tel},
    Form, FormidableServerAction,
};
use leptos::server_fn::codec::Json;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use url::Url;

use crate::app::i18n::*;

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserForm {
    #[form(
        label = personal_information,
        description = "Please provide your personal details."
    )]
    personal_info: PersonalInfo,
    #[form(
        label = contact_information,
    )]
    contact_info: ContactInfo,
    #[form(label = "Addresses")]
    addresses: Vec<Address>,
    #[form(label = user_type)]
    user_type: UserType,
    #[form(label = "Account Balance")]
    account_balance: bigdecimal::BigDecimal,
    #[form(label = "Accept Terms")]
    accept_terms: Accept,
    #[form(label = "Payment Method")]
    payment_method: PaymentMethod,
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

#[derive(Form, Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
enum UserType {
    #[form(label = "Admin")]
    Admin,
    #[form(label = regular_user)]
    #[default]
    Regular,
}

#[derive(Form, Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
enum Country {
    #[form(label = switzerland)]
    #[default]
    Switzerland,
    #[form(label = "Germany")]
    Germany,
    #[form(label = "France")]
    France,
    #[form(label = "Italy")]
    Italy,
    #[form(label = "Spain")]
    Spain,
    #[form(label = "Portugal")]
    Portugal,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
enum PaymentMethod {
    #[form(label = "Credit Card")]
    CreditCard(String), // Card number as a single unnamed field
    #[form(label = "Bank Transfer")]
    BankTransfer {
        #[form(label = "Account Number")]
        account_number: String,
        #[form(label = "Routing Number")]
        routing_number: String,
    },
    #[form(label = "Cash")]
    #[default]
    Cash,
}

#[component]
pub fn ExampleForm() -> impl IntoView {
    let i18n = use_i18n();

    let on_switch = move |_| {
        let new_locale = match i18n.get_locale() {
            Locale::en => Locale::de,
            Locale::de => Locale::en,
        };
        i18n.set_locale(new_locale);
    };

    view! {
        <button on:click=on_switch>{t!(i18n, personal_information)}</button>
        <FormidableServerAction<HandleUserForm, UserForm> label="Example Form" name="user_form" />
    }
}

#[server(
  input = Json,
  output = Json
)]
async fn handle_user_form(user_form: UserForm) -> Result<(), ServerFnError> {
    leptos::logging::log!("Received form submission: {:?}", user_form);
    Ok(())
}
