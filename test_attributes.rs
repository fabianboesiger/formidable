use formidable::Form;
use strum::{Display, IntoStaticStr, VariantArray};

const DYNAMIC_LABEL: &str = "Dynamic Label";
const DYNAMIC_DESC: &str = "Dynamic Description";

#[derive(Form)]
struct TestAttributes {
    // String literal
    #[form(label = "String Label", description = "This is a string description")]
    string_field: String,

    // Identifier reference
    #[form(label = DYNAMIC_LABEL, description = DYNAMIC_DESC)]
    ident_field: String,

    // Arithmetic expression
    #[form(label = 1 + 1, description = "Math result as label")]
    expr_field: String,

    // Multiple attributes
    #[form(
        label = "Multi Field",
        description = "This has both label and description"
    )]
    multi_field: String,

    // Just description
    #[form(description = "Only description here")]
    desc_only_field: String,

    // Default (no attributes)
    default_field: String,

    // Test enum with form labels
    user_type: UserType,
}

#[derive(Form, Clone, Debug, PartialEq)]
enum UserType {
    #[form(label = "Individual User")]
    Individual,

    #[form(label = "Business Account")]
    Business {
        #[form(label = "Company Name")]
        company_name: String,
        #[form(label = "Tax ID")]
        tax_id: String,
    },

    #[form(label = "Premium Member")]
    Premium(String), // subscription level

    // This one will use the variant name as default
    Guest,
}
