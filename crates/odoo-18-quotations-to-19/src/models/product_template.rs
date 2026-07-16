use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

pub const PRODUCT_TEMPLATE_MODEL_NAME: &str = "product.template";

#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    Default,
)]
#[serde(rename_all = "snake_case")]
pub enum ProductTemplateType {
    #[display("Goods")]
    #[default]
    Consu,
    #[display("Service")]
    Service,
    #[display("Combo")]
    Combo,
}
#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    Default,
)]
#[serde(rename_all = "snake_case")]
pub enum ProductServiceTracking {
    #[default]
    #[display("Nothing")]
    No,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice, Serialize)]
pub struct ProductTemplateFromOdoo18 {
    pub id: u64,

    pub name: String,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub description_purchase: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub description_sale: Option<String>,
    #[field_names_as_slice(skip)]
    pub type_: ProductTemplateType,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub combo_ids: Vec<u64>,
    pub service_tracking: ProductServiceTracking,
    pub categ_id: Many2OneRepr,

    // Currencies
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub currency_id: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub cost_currency_id: Option<Many2OneRepr>,

    // Price
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub list_price: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub standard_price: Option<f32>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub volume: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub weight: Option<f32>,

    pub sale_ok: bool,
    pub purchase_ok: bool,
    pub uom_id: Many2OneRepr,
    pub uom_po_unit: Many2OneRepr,

    pub color: u16,

    pub attribute_line_ids: Vec<u64>,

    pub valid_product_template_attribute_line_ids: Vec<u64>,

    pub product_variant_ids: Vec<u64>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub barcode: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub default_code: Option<String>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_document_ids: Vec<String>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_tag_ids: Vec<u64>,
    // TODO do product properties
}

pub fn product_template_from_odoo_18() -> Vec<String> {
    let mut fields = ProductTemplateFromOdoo18::FIELD_NAMES_AS_SLICE
        .iter()
        .map(|a| String::from(*a))
        .collect::<Vec<String>>();
    fields.push("type".into());
    fields
}

impl ModelName for ProductTemplateFromOdoo18 {
    const NAME: &'static str = PRODUCT_TEMPLATE_MODEL_NAME;
}
