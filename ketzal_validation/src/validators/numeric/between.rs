use crate::errors::ValidationErrors;
use crate::i18n;
use crate::traits::Validator;
use crate::utils::helpers;

pub struct Between;

impl Validator for Between {
    fn name(&self) -> &'static str {
        "between"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        args: &[String],
    ) -> Result<(), ValidationErrors> {
        let min = args.first().ok_or_else(|| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "between", i18n::t("validator.between.arg_required", &[]));
            errors
        })?.parse::<f64>().map_err(|_| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "between", i18n::t("validator.between.arg_not_number", &[]));
            errors
        })?;

        let max = args.get(1).ok_or_else(|| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "between", i18n::t("validator.between.arg_required", &[]));
            errors
        })?.parse::<f64>().map_err(|_| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "between", i18n::t("validator.between.arg_not_number", &[]));
            errors
        })?;

        if min > max {
            let mut errors = ValidationErrors::new();
            errors.push(field, "between", i18n::t("validator.between.invalid_range", &[]));
            return Err(errors);
        }

        if let Some(s) = value.downcast_ref::<String>() {
            let len = s.len() as f64;
            if len < min || len > max {
                let mut errors = ValidationErrors::new();
                errors.push(
                    field,
                    "between",
                    i18n::t("validator.between.not_between", &[("field", field), ("min", &min.to_string()), ("max", &max.to_string())]),
                );
                return Err(errors);
            }
            return Ok(());
        }

        let Some(num_val) = helpers::cast::any_to_f64(value) else {
            return Err(helpers::unsupported_type::unsupported_type(field, "between"));
        };

        if num_val < min || num_val > max {
            let mut errors = ValidationErrors::new();
            errors.push(
                field,
                "between",
                i18n::t("validator.between.not_between", &[("field", field), ("min", &min.to_string()), ("max", &max.to_string())]),
            );
            return Err(errors);
        }

        Ok(())
    }
}
