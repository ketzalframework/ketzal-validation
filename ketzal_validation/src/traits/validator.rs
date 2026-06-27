use crate::errors::ValidationErrors;

pub trait Validator: Send + Sync {
    fn name(&self) -> &'static str;

    fn validate(
        &self,
        field: &str,
        value: &dyn std::any::Any,
        args: &[String],
    ) -> Result<(), ValidationErrors>;
}
