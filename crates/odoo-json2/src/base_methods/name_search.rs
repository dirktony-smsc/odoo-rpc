use odoo_api_commons::Domain;
use serde::Serialize;

#[derive(Debug, Serialize, Default, Clone)]
pub struct NameSearchParam {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domain: Vec<Domain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}
