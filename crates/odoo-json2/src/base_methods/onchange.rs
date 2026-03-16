use better_default::Default;
use serde::Serialize;

use crate::utils;

#[derive(Debug, Serialize, Default, Clone)]
pub struct OnchangeParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    pub ids: Vec<u64>,
    #[default(utils::empty_dict())]
    pub values: serde_json::Value,
    pub field_names: Vec<String>,
    #[default(utils::empty_dict())]
    pub fields_spec: serde_json::Value,
}
