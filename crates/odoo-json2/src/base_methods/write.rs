use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct WriteParam<T> {
    pub ids: Vec<u64>,
    pub vals: T,
}
