use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

#[derive(Debug, Deserialize, FieldNamesAsSlice, Serialize, Clone)]
pub struct ProductTagFromOdoo18 {
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub sequence: Option<u16>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub color: Option<String>,
}

impl ModelName for ProductTagFromOdoo18 {
    const NAME: &'static str = "product.tag";
}

#[derive(Debug, Serialize, Clone)]
pub struct ProductTagToOdoo19 {
    pub name: String,
    pub sequence: Option<u16>,
    pub color: Option<String>,
    pub visible_to_customers: bool,
}
