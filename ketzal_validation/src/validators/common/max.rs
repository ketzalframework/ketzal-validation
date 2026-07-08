use crate::errors::ValidationErrors;
use crate::utils::helpers;
use crate::traits::Validator;

pub struct Max;

impl Validator for Max {
    fn name(&self) -> &'static str {
        "max"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        args: &[String],
    ) -> Result<(), ValidationErrors> {
        let max = helpers::parse_arg_f64(field, "max", args)?;
        helpers::validate_threshold(field, value, "max", max, |a, b| a > b, "validator.max.exceeded")
    }
}
