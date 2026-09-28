use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub odoo: OdooClientConfig,
    pub ssl_config: Option<SslConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SslConfig {
    #[serde(default)]
    pub additional_certs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct OdooClientConfig {
    pub url: url::Url,
    pub api_key: String,
    pub database: Option<String>,
    pub host: Option<String>,
    pub user_agent: Option<String>,
}
