use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum DisplayType {
    #[display("Section")]
    LineSection,
    #[display("Note")]
    LineNote,
}

pub const SALES_ORDER_LINE_MODEL_NAME: &str = "sale.order.line";

#[derive(Debug, Clone, Deserialize, FieldNamesAsSlice)]
pub struct SalesOrderLineFrom18 {
    pub id: u64,
    pub active: bool,

    pub order_id: Many2OneRepr,
    pub sequence: u32,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub display_type: Option<DisplayType>,
    pub is_downpayement: bool,
    pub is_expense: bool,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_id: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_template_id: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_uom_category_id: Option<Many2OneRepr>,

    pub product_uom_qty: f32,
    pub product_uom: Many2OneRepr,
    pub linked_line_id: Many2OneRepr,

    pub tax_id: Many2OneRepr,
    pub price_unit: f32,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub customer_lead: Option<f32>,
}

impl ModelName for SalesOrderLineFrom18 {
    const NAME: &'static str = SALES_ORDER_LINE_MODEL_NAME;
}
