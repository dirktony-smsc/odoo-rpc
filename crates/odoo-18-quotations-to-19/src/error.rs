#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    Rpc(#[from] odoo_rpc::error::Error),
    Json2(#[from] odoo_json2::error::Error),
    #[error("not found??")]
    NotFound,
    #[error("Nothing created when creating objects")]
    NothingCreated,
    SerdeJson(#[from] serde_json::Error),
    Regex(#[from] regex::Error),
    StdIo(#[from] std::io::Error),
    Reqwest(#[from] reqwest::Error),
    #[error("Properties ID not found for {}", .0)]
    PropertiesNotFound(String),
    X19PropertyNameParseError(
        #[from] crate::transfert::res_partner_18_fields_to_19_properties::X19PropertyNameParseError,
    ),
}
