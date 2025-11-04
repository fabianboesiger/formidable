use leptos::prelude::*;

use formidable::{
    types::{Accept, Color, Date, Email, File, NonEmptyString, Optional, Tel},
    Form, FormidableServerAction,
};
use leptos::server_fn::codec::Json;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use url::Url;

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct FormData {
    #[form(
        label = personal_information,
        description = "Please provide your personal details."
    )]
    personal_info: PersonalInfo,
    #[form(label = "Contact Information")]
    contact_info: ContactInfo,
    #[form(label = "Order")]
    order: Vec<Item>,
    #[form(label = "Payment Information")]
    payment_info: Payment,
    #[form(label = "I accept the terms and conditions")]
    terms_and_conditions: Accept,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct PersonalInfo {
    #[form(label = "Full Name")]
    name: NonEmptyString,
    #[form(label = "Date of Birth")]
    date_of_birth: Date,
    #[form(label = "Eye Color")]
    eye_color: Color,
    #[form(label = "ID Document")]
    id: File,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ContactInfo {
    #[form(label = "Address")]
    address: Address,
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

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Item {
    #[form(label = "Item Name")]
    name: String,
    #[form(label = "Item ID")]
    id: u32,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Payment {
    #[form(label = "Total")]
    total: bigdecimal::BigDecimal,
    #[form(label = "Payment Method")]
    payment_method: PaymentMethod,
}

#[derive(Form, Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
enum Country {
    #[form(label = "Switzerland")]
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
    CreditCard(String),
    #[form(label = "Bank Transfer")]
    BankTransfer {
        #[form(label = "Account Number")]
        account_number: String,
        #[form(label = "Routing Number")]
        routing_number: String,
    },
    #[default]
    #[form(label = "Cash")]
    Cash,
}

#[component]
pub fn ExampleForm() -> impl IntoView {
    view! {
        <FormidableServerAction<HandleSubmit, FormData> label="Example Form" name="user_form" />
    }
}

#[server(
  input = Json,
  output = Json
)]
async fn handle_submit(user_form: FormData) -> Result<(), ServerFnError> {
    leptos::logging::log!("Received form submission: {:?}", user_form);
    Ok(())
}
