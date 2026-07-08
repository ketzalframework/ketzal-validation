use crate::errors::ValidationErrors;
use crate::i18n;
use crate::traits::Validator;

pub struct Integer;

impl Validator for Integer {
    fn name(&self) -> &'static str {
        "integer"
    }

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        _args: &[String],
    ) -> Result<(), ValidationErrors> {
        if value.downcast_ref::<i32>().is_some()
            || value.downcast_ref::<i64>().is_some()
            || value.downcast_ref::<u32>().is_some()
            || value.downcast_ref::<u64>().is_some()
        {
            return Ok(());
        }

        if let Some(v) = value.downcast_ref::<f32>() {
            if v.fract() == 0.0 {
                return Ok(());
            }
        }

        if let Some(v) = value.downcast_ref::<f64>() {
            if v.fract() == 0.0 {
                return Ok(());
            }
        }

        if let Some(s) = value.downcast_ref::<String>() {
            if s.parse::<i64>().is_ok() {
                return Ok(());
            }
        }

        let mut errors = ValidationErrors::new();
        errors.push(field, "integer", i18n::t("validator.integer.not_integer", &[("field", field)]));
        Err(errors)
    }
}
