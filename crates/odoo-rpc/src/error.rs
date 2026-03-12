#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum Error {
    RPCClient(#[from] jsonrpsee::core::ClientError),
    #[error("The current client is not logged in")]
    NotLoggedIn,
    SerdeJson(#[from] serde_json::Error),
}
