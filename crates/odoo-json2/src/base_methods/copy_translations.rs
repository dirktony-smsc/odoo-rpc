use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CopyTranslationsParam {
    pub ids: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    pub new: String,
    pub excluded: Vec<String>,
}
