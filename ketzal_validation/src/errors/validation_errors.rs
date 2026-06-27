use serde::Serialize;

use super::ValidationError;

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn push(
        &mut self,
        field: impl Into<String>,
        rule: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.errors.push(ValidationError {
            field: field.into(),
            rule: rule.into(),
            message: message.into(),
        });
    }

    pub fn extend(&mut self, errors: ValidationErrors) {
        self.errors.extend(errors.errors);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}
