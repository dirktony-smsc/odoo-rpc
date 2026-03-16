use serde::Serialize;

use odoo_api_commons::Domain;

#[derive(Debug, Serialize, Default, Clone)]
pub struct SearchCountParam {
    pub domain: Vec<Domain>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}
