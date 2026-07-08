use crate::errors::ValidationErrors;
use crate::i18n;

use super::cast;
use super::unsupported_type::unsupported_type;

pub(crate) fn validate_numeric(
    field: &str,
    value: &dyn std::any::Any,
    rule: &str,
    is_invalid: fn(f64) -> bool,
    error_key: &str,
) -> Result<(), ValidationErrors> {
    let Some(num_val) = cast::any_to_f64(value) else {
        return Err(unsupported_type(field, rule));
    };
    if is_invalid(num_val) {
        let mut errors = ValidationErrors::new();
        errors.push(field, rule, i18n::t(error_key, &[("field", field)]));
        return Err(errors);
    }
    Ok(())
}
