use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
pub struct FieldsGetParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    /// Fields that you want to get the information from
    pub allfields: Vec<String>,
    /// the attributes that you want to get
    pub attributes: Option<Vec<String>>,
}
