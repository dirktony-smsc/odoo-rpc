#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    RPCClient(#[from] jsonrpsee::core::ClientError),
}
