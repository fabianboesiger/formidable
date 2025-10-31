use crate::{components::InputType, types::FormType};

impl FormType for bigdecimal::BigDecimal {
    const INPUT_TYPE: InputType = InputType::Text;
    const REQUIRED: Option<bool> = Some(true);
}
