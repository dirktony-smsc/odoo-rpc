use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct DefaultGetParam {
    pub fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}
