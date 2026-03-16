use odoo_api_commons::deserialize_and_default_if_false;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Default)]
pub struct GetMetadataParam {
    pub ids: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct XmlIdInfo {
    pub xmlid: String,
    pub noupdate: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ObjectMetadata {
    pub id: u64,
    pub create_uid: (u64, String),
    pub create_date: String,
    pub write_uid: (u64, String),
    pub write_date: String,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub xmlid: Option<String>,
    pub noupdate: bool,
    pub xmlids: Vec<XmlIdInfo>,
}
