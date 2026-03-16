use odoo_api_commons::deserialize_and_default_if_false;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Default)]
pub struct LoadParam {
    pub data: serde_json::Value,
    pub fields: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoadCallOut {
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub ids: Vec<u64>,
    pub messages: Vec<serde_json::Value>,
    pub nextrow: Option<u64>,
    pub lastrow: Option<u64>,
}
