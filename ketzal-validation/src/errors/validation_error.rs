use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ValidationError {
    pub field: String,

    pub rule: String,

    pub message: String,
}
