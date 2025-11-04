# Formidable

With formidable, its possible to easily derive Leptos forms for structs and enums.

## Example

There is a fully featured example project available in this repository [here](https://github.com/fabianboesiger/formidable/tree/main/formidable-example).

```rust
#[derive(Form, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct FormData {
    #[form(
        label = "Personal",
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

// ...

#[component]
pub fn ExampleForm() -> impl IntoView {
    view! {
        <FormidableServerAction<HandleSubmit, FormData>
            label="Example Form"
            name="user_form" />
    }
}

#[server(
  input = Json,
  output = Json
)]
async fn handle_submit(user_form: FormData) -> Result<(), ServerFnError> {
    leptos::logging::log!("Received form: {:?}", user_form);
    Ok(())
}

```

## Features

- Support for structs via derive macro
- Support for enums via derive macro
    - Unit enums are rendered as a radio button or select
    - Unnamed and named enums show a further form section to capture the required enum variant data
- Type-based validation approach, easily add validation with the newtype pattern
    - Supports types from the crates `time`, `url`, `color`, `bigdecimal`
    - Provides further types for email, phone number, non empty strings
    - Supports dynamically repeating elements via `Vec`
- Supports i18n support via `leptos_i18n`
- Send your data to the server directly via server actions, or get your data via callbacks

## Adding Custom Types

Formidable uses a type-based validation approach. Types provide the validation mechanism which are then enforced in the form inputs. You can easily extend the system using the `FromStr` and `FormType` traits.

```rust
// Define a new type for validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Into, Deref, Serialize, Deserialize)]
pub struct Email(String);

// Add a custom error type
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq, Hash)]
pub enum EmailError {
    #[error("Invalid format")]
    InvalidFormat,
}

// Implement `FromStr` for validation
impl FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('@') {
            Ok(Email(s.to_string()))
        } else {
            Err(EmailError::InvalidFormat)
        }
    }
}

// Implement `FormType` to use the `Email` type in derived forms
impl FormType for Email {
    // The HTML input type to use
    const INPUT_TYPE: InputType = InputType::Email;
    // ... other HTML options such as required, minlength, maxlength, ...
    const REQUIRED: Option<bool> = Some(true);
}

```

It's also possible to add more sophisticated input fields by implementing `Form` directly:

```rust
impl Form for bool {
    fn view(
        field: FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <Checkbox<bool>
                label=field.label.expect("No label provided")
                description=field.description
                name=name
                value=value
                callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                    callback.run(v.map_err(FormError::from));
                }))}
            />
        }
    }
}

```

## Enum Support

Support for enums are a core feature of this crate and provide an easy way to create form inputs with "either or" logic.

```rust
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
```