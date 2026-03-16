use serde::Serialize;

use odoo_api_commons::{Domain, PaginationParam};

#[derive(Debug, Serialize, Default, Clone)]
pub struct SearchReadParam {
    pub domain: Vec<Domain>,
    pub fields: Vec<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}
