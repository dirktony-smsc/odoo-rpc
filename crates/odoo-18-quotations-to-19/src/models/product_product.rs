use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

pub const PRODUCT_PRODUCT_MODEL_NAME: &str = "product.product";

#[derive(Debug, Deserialize, FieldNamesAsSlice, Serialize)]
pub struct ProductProductFromOdoo18 {
    pub id: u64,

    pub product_tmpl_id: Many2OneRepr,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub barcode: Option<String>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub standard_price: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub volume: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub weight: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_document_ids: Vec<u64>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub image_variant_1920: Option<String>,
}

impl ModelName for ProductProductFromOdoo18 {
    const NAME: &'static str = PRODUCT_PRODUCT_MODEL_NAME;
}
