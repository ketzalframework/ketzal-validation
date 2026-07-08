use crate::errors::ValidationErrors;
use crate::i18n;

use super::cast;
use super::unsupported_type::unsupported_type;

/// Valida un valor contra un umbral, soportando `String` (por longitud) y numéricos.
///
/// Útil para validadores con argumento como `min` o `max`.
/// Si el valor es `String` compara su longitud, si es numérico lo convierte a `f64`.
pub(crate) fn validate_threshold(
    field: &str,
    value: &dyn std::any::Any,
    rule: &str,
    threshold: f64,
    is_invalid: fn(f64, f64) -> bool,
    error_key: &str,
) -> Result<(), ValidationErrors> {
    if let Some(s) = value.downcast_ref::<String>() {
        if is_invalid(s.len() as f64, threshold) {
            let mut errors = ValidationErrors::new();
            errors.push(
                field,
                rule,
                i18n::t(
                    error_key,
                    &[("field", field), (rule, &threshold.to_string())],
                ),
            );
            return Err(errors);
        }
        return Ok(());
    }

    let Some(num_val) = cast::any_to_f64(value) else {
        return Err(unsupported_type(field, rule));
    };

    if is_invalid(num_val, threshold) {
        let mut errors = ValidationErrors::new();
        errors.push(
            field,
            rule,
            i18n::t(
                error_key,
                &[("field", field), (rule, &threshold.to_string())],
            ),
        );
        return Err(errors);
    }

    Ok(())
}
