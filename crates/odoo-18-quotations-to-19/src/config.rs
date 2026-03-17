use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Odoo18Config {
    pub url: Url,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Odoo19Config {
    pub url: Url,
    pub api_key: String,
    pub database: Option<String>,
    pub host: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Config {
    pub odoo_18: Odoo18Config,
    pub odoo_19: Odoo19Config,
}
