use crate::components::InputType;
use crate::types::FormType;

impl FormType for url::Url {
    const INPUT_TYPE: InputType = InputType::Url;
    const REQUIRED: Option<bool> = Some(true);
}
