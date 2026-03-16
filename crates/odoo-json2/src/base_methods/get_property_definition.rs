use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
pub struct GetPropertyDefinitionParam {
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}
