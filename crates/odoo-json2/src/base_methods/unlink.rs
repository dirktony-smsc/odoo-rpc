use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct UnlinkParam {
    pub ids: Vec<u64>,
}
