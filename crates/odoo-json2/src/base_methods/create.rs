use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CreateParam<T> {
    pub vals_list: Vec<T>,
}
