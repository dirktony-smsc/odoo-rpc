use serde::Serialize;

use crate::utils::{Domain, PaginationParam};

#[derive(Debug, Serialize, Default)]
pub struct SearchParam {
    pub domain: Vec<Domain>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationParam>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}
