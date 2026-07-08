use crate::errors::ValidationErrors;
use crate::utils::helpers;
use crate::traits::Validator;

pub struct Negative;

impl Validator for Negative {
    fn name(&self) -> &'static str {
        "negative"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        _args: &[String],
    ) -> Result<(), ValidationErrors> {
        helpers::validate_numeric(field, value, "negative", |v| v >= 0.0, "validator.negative.not_negative")
    }
}
