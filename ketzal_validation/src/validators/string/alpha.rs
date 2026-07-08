use crate::errors::ValidationErrors;
use crate::i18n;
use crate::traits::Validator;

pub struct Alpha;

impl Validator for Alpha {
    fn name(&self) -> &'static str {
        "alpha"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        _args: &[String],
    ) -> Result<(), ValidationErrors> {
        let Some(s) = value.downcast_ref::<String>() else {
            let mut errors = ValidationErrors::new();
            errors.push(field, "alpha", i18n::t("unsupported_type", &[]));
            return Err(errors);
        };

        if !s.chars().all(|c| c.is_alphabetic()) {
            let mut errors = ValidationErrors::new();
            errors.push(field, "alpha", i18n::t("validator.alpha.not_alpha", &[("field", field)]));
            return Err(errors);
        }

        Ok(())
    }
}
