use crate::errors::ValidationErrors;
use crate::utils::helpers;
use crate::traits::Validator;

pub struct Min;

impl Validator for Min {
    fn name(&self) -> &'static str {
        "min"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        args: &[String],
    ) -> Result<(), ValidationErrors> {
        let min = helpers::parse_arg_f64(field, "min", args)?;
        helpers::validate_threshold(field, value, "min", min, |a, b| a < b, "min.below")
    }
}
