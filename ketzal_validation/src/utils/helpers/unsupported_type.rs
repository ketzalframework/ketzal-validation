use crate::errors::ValidationErrors;
use crate::i18n;

pub(crate) fn unsupported_type(field: &str, rule: &str) -> ValidationErrors {
    let mut errors = ValidationErrors::new();
    errors.push(field, rule, i18n::t("validator.unsupported_type", &[]));
    errors
}
