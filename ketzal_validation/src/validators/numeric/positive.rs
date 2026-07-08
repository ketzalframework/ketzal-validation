use crate::errors::ValidationErrors;
use crate::utils::helpers;
use crate::traits::Validator;

pub struct Positive;

impl Validator for Positive {
    fn name(&self) -> &'static str {
        "positive"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        _args: &[String],
    ) -> Result<(), ValidationErrors> {
        helpers::validate_numeric(field, value, "positive", |v| v <= 0.0, "validator.positive.not_positive")
    }
}
