#[derive(Debug, Clone)]
pub struct OpenApiSchema {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub example: Option<serde_json::Value>,
    pub read_only: bool,
    pub write_only: bool,
    pub default: Option<serde_json::Value>,
    pub required: Vec<String>,
    pub deprecated: bool,
}
