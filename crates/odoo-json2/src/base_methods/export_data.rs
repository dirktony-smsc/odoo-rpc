use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct ExportDataParam {
    pub ids: Vec<u64>,
    pub fields_to_export: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExportDataOutput {
    pub datas: Vec<Vec<serde_json::Value>>,
}
