use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Version {
    pub protocol_version: u32,
    pub server_serie: String,
    pub server_version: String,
    pub server_version_info: Vec<serde_json::Value>,
}
