use derive_more::derive::Display;
use odoo_api_commons::{Command, deserialize_and_default_if_false};
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

    pub name: String,

    pub order_id: Many2OneRepr,
    pub sequence: u32,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub display_type: Option<DisplayType>,
    pub is_downpayment: bool,
    pub is_expense: bool,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_id: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_template_id: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_uom_category_id: Option<Many2OneRepr>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_uom_qty: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_uom: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub linked_line_id: Option<Many2OneRepr>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub tax_id: Option<Vec<u64>>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub price_unit: Option<f32>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub customer_lead: Option<f32>,
}

impl ModelName for SalesOrderLineFrom18 {
    const NAME: &'static str = SALES_ORDER_LINE_MODEL_NAME;
}

#[derive(Debug, Clone, Serialize)]
pub struct SalesOrderLineToOdoo19 {
    pub name: String,

    pub order_id: u64,
    pub sequence: u32,

    pub display_type: Option<DisplayType>,
    pub is_downpayment: bool,
    pub is_expense: bool,

    pub product_id: Option<u64>,
    pub product_template_id: Option<u64>,
    pub product_uom_category_id: Option<u64>,

    pub product_uom_qty: Option<f32>,
    pub product_uom: Option<u64>,
    pub linked_line_id: Option<u64>,

    pub tax_id: Option<Vec<Command<()>>>,
    pub price_unit: Option<f32>,

    pub customer_lead: Option<f32>,
}
