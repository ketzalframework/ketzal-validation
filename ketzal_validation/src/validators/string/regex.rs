use regex::Regex;
use crate::errors::ValidationErrors;
use crate::i18n;
use crate::traits::Validator;

pub struct RegexValidator;

impl Validator for RegexValidator {
    fn name(&self) -> &'static str {
        "regex"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        args: &[String],
    ) -> Result<(), ValidationErrors> {
        let Some(s) = value.downcast_ref::<String>() else {
            let mut errors = ValidationErrors::new();
            errors.push(field, "regex", i18n::t("validator.unsupported_type", &[]));
            return Err(errors);
        };

        let pattern = args.first().ok_or_else(|| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "regex", i18n::t("regex.arg_required", &[]));
            errors
        })?;

        let re = Regex::new(pattern).map_err(|_| {
            let mut errors = ValidationErrors::new();
            errors.push(field, "regex", i18n::t("regex.invalid_pattern", &[("field", field)]));
            errors
        })?;

        if !re.is_match(s) {
            let mut errors = ValidationErrors::new();
            errors.push(field, "regex", i18n::t("regex.no_match", &[("field", field)]));
            return Err(errors);
        }

        Ok(())
    }
}
