use std::any::Any;

use crate::{errors::ValidationErrors, traits::Validator};

pub struct MaxValidator;

impl Validator for MaxValidator {
    fn name(&self) -> &'static str {
        "max"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn Any,
        args: &[String],
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        let max = args
            .first()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0);

        if let Some(value) = value.downcast_ref::<i64>() {
            if *value > max {
                errors.push(field, "max", format!("Must not be greater than {}", max));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
