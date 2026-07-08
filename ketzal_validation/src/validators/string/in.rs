use crate::errors::ValidationErrors;
use crate::i18n;
use crate::traits::Validator;

pub struct In;

impl Validator for In {
    fn name(&self) -> &'static str {
        "in"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        args: &[String],
    ) -> Result<(), ValidationErrors> {
        let Some(s) = value.downcast_ref::<String>() else {
            let mut errors = ValidationErrors::new();
            errors.push(field, "in", i18n::t("unsupported_type", &[]));
            return Err(errors);
        };

        if !args.contains(s) {
            let mut errors = ValidationErrors::new();
            errors.push(
                field,
                "in",
                i18n::t("validator.in.not_in_list", &[("field", field)]),
            );
            return Err(errors);
        }

        Ok(())
    }
}

pub struct NotIn;

impl Validator for NotIn {
    fn name(&self) -> &'static str {
        "not_in"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        args: &[String],
    ) -> Result<(), ValidationErrors> {
        let Some(s) = value.downcast_ref::<String>() else {
            let mut errors = ValidationErrors::new();
            errors.push(field, "not_in", i18n::t("unsupported_type", &[]));
            return Err(errors);
        };

        if args.contains(s) {
            let mut errors = ValidationErrors::new();
            errors.push(
                field,
                "not_in",
                i18n::t("validator.not_in.in_list", &[("field", field)]),
            );
            return Err(errors);
        }

        Ok(())
    }
}
