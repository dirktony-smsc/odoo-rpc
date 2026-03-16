use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CopyDataParam<T = ()> {
    pub ids: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<T>,
}
