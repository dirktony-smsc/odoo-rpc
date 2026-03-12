use serde::{Deserialize, Serialize};

/// Reference: https://www.odoo.com/documentation/19.0/developer/reference/external_api.html#common-service
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct OdooVersion {
    pub version_info: Vec<serde_json::Value>,
    pub version: String,
}
