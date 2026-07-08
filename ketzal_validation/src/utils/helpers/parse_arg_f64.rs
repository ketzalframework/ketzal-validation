use crate::errors::ValidationErrors;
use crate::i18n;

pub(crate) fn parse_arg_f64(
    field: &str,
    rule: &str,
    args: &[String],
) -> Result<f64, ValidationErrors> {
    let arg = args.first().ok_or_else(|| {
        let mut errors = ValidationErrors::new();
        errors.push(field, rule, i18n::t(&format!("{rule}.arg_required"), &[]));
        errors
    })?;
    arg.parse().map_err(|_| {
        let mut errors = ValidationErrors::new();
        errors.push(field, rule, i18n::t(&format!("{rule}.arg_not_number"), &[]));
        errors
    })
}
