use crate::errors::ValidationErrors;
use crate::i18n;
use crate::traits::Validator;
use std::any::Any;

pub struct Email;

impl Email {
    fn is_valid_email(email: &str) -> bool {
        let email = email.trim();

        if email.is_empty() || email.contains(' ') {
            return false;
        }

        let mut parts = email.split('@');

        let local = match parts.next() {
            Some(v) if !v.is_empty() => v,
            _ => return false,
        };

        let domain = match parts.next() {
            Some(v) if !v.is_empty() => v,
            _ => return false,
        };

        if parts.next().is_some() {
            return false;
        }

        if local.starts_with('.') || local.ends_with('.') {
            return false;
        }

        if domain.starts_with('.') || domain.ends_with('.') {
            return false;
        }

        domain.contains('.')
    }
}

impl Validator for Email {
    fn name(&self) -> &'static str {
        "email"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn Any,
        _args: &[String],
    ) -> Result<(), ValidationErrors> {
        let Some(email) = value.downcast_ref::<String>() else {
            let mut errors = ValidationErrors::new();

            errors.push(
                field,
                self.name(),
                i18n::t("validator.unsupported_type", &[]),
            );

            return Err(errors);
        };

        if !Self::is_valid_email(email) {
            let mut errors = ValidationErrors::new();

            errors.push(
                field,
                self.name(),
                i18n::t("email.invalid", &[("field", field)]),
            );

            return Err(errors);
        }

        Ok(())
    }
}
