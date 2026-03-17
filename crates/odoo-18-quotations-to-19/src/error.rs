#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    Rpc(#[from] odoo_rpc::error::Error),
    Json2(#[from] odoo_json2::error::Error),
}
