use serde::Serialize;

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct PaginationParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
