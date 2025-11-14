use leptos::prelude::*;

use formidable::{
    types::{Accept, Color, Date, Email, File, NonEmptyString, Optional, Tel},
    Form, FormConfiguration, FormidableServerAction, SectionHeading,
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
#[form(class = "personal-info-section")]
struct PersonalInfo {
    #[form(label = "Full Name", class = "special-input")]
    name: NonEmptyString,
    #[form(label = "Date of Birth")]
    date_of_birth: Date,
    #[form(label = "Eye Color")]
    eye_color: Color,
    #[form(label = "ID Document")]
    id: File,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[form(columns = 2)]
struct ContactInfo {
    #[form(label = "Address", colspan = 2)]
    address: Address,
    #[form(label = "Email Address")]
    email: Email,
    #[form(label = "Phone Number")]
    phone: Tel,
    #[form(label = "Website", colspan = 2)]
    website: Optional<Url>,
}

#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[form(columns = 12)]
struct Address {
    #[form(label = "Street", colspan = 8)]
    street: String,
    #[form(label = "House Number", colspan = 4)]
    house_number: String,
    #[form(label = "Zip Code", colspan = 4)]
    zip: String,
    #[form(label = "City", colspan = 8)]
    city: String,
    #[form(label = "State", colspan = 8)]
    state: String,
    #[form(label = "Country", colspan = 4)]
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
#[form(variant_selection = "select", class = "country-selector")]
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
#[form(variant_selection = "radio", class = "payment-method-selector")]
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
        <FormidableServerAction<HandleSubmit, FormData>
            label="Example Form"
            description="An example form demonstrating various field types and configurations."
            name="user_form"
            form_configuration=FormConfiguration {
                section_label: SectionHeading::LeveledHeading(formidable::HeadingLevel::H2),
            }
        />
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
