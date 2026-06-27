use crate::errors::ValidationErrors;
use crate::i18n;
use crate::traits::Validator;

use super::cast;

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
        let max = args.first().ok_or_else(|| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "max", i18n::t("max.arg_required", &[]));
            errors
        })?;

        let max_val: f64 = max.parse().map_err(|_| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "max", i18n::t("max.arg_not_number", &[]));
            errors
        })?;

        if let Some(s) = value.downcast_ref::<String>() {
            if (s.len() as f64) > max_val {
                let mut errors = ValidationErrors::new();
                errors.push(
                    field,
                    "max",
                    i18n::t("max.exceeded", &[("field", field), ("max", max)]),
                );
                return Err(errors);
            }
            return Ok(());
        }

        let Some(num_val) = cast::any_to_f64(value) else {
            let mut errors = ValidationErrors::new();
            errors.push(field, "max", i18n::t("max.unsupported_type", &[]));
            return Err(errors);
        };

        if num_val > max_val {
            let mut errors = ValidationErrors::new();
            errors.push(
                field,
                "max",
                i18n::t("max.exceeded", &[("field", field), ("max", max)]),
            );
            return Err(errors);
        }

        Ok(())
    }
}
