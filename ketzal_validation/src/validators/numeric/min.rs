use std::any::Any;

use crate::{errors::ValidationErrors, traits::Validator};

pub struct MinValidator;

impl Validator for MinValidator {
    fn name(&self) -> &'static str {
        "min"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn Any,
        args: &[String],
    ) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        let min = args
            .first()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0);

        if let Some(value) = value.downcast_ref::<i64>() {
            if *value < min {
                errors.push(field, "min", format!("Must be at least {}", min));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
