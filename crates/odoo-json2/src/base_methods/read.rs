use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ReadParam {
    pub ids: Vec<u64>,
    pub fields: Vec<String>,
}
