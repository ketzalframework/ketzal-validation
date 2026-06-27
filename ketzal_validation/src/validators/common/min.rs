use crate::errors::ValidationErrors;
use crate::i18n;
use crate::traits::Validator;

use super::cast;

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
        let min = args.first().ok_or_else(|| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "min", i18n::t("min.arg_required", &[]));
            errors
        })?;

        let min_val: f64 = min.parse().map_err(|_| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "min", i18n::t("min.arg_not_number", &[]));
            errors
        })?;

        if let Some(s) = value.downcast_ref::<String>() {
            if (s.len() as f64) < min_val {
                let mut errors = ValidationErrors::new();
                errors.push(
                    field,
                    "min",
                    i18n::t("min.below", &[("field", field), ("min", min)]),
                );
                return Err(errors);
            }
            return Ok(());
        }

        let Some(num_val) = cast::any_to_f64(value) else {
            let mut errors = ValidationErrors::new();
            errors.push(field, "min", i18n::t("min.unsupported_type", &[]));
            return Err(errors);
        };

        if num_val < min_val {
            let mut errors = ValidationErrors::new();
            errors.push(
                field,
                "min",
                i18n::t("min.below", &[("field", field), ("min", min)]),
            );
            return Err(errors);
        }

        Ok(())
    }
}
