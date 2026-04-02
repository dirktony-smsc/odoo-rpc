use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

pub const ACCOUNT_MOVE_LINE_MODEL_NAME: &str = "account.move.line";

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountMoveLineDisplayType {
    #[display("Draft")]
    Product,
    #[display("Cost of Goods Sold")]
    Cogs,
    #[display("Tax")]
    Tax,
    #[display("Discount")]
    Discount,
    #[display("Rounding")]
    Rounding,
    #[display("Payment Term")]
    PaymentTerm,
    #[display("Section")]
    LineSection,
    #[display("Note")]
    LineNote,
    #[display("Early Payment Discount")]
    Epd,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice, Serialize)]
pub struct AccountMoveLineFromOdoo18 {
    pub id: u64,

    // Accounting fields
    pub move_id: Many2OneRepr,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub debit: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub credit: Option<f32>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub currency_id: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub partner_id: Option<Many2OneRepr>,

    pub display_type: AccountMoveLineDisplayType,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub product_id: Option<Many2OneRepr>,
    pub quantity: f32,
    pub price_unit: f32,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub discount: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub account_id: Option<Many2OneRepr>,

    pub is_refund: bool,
}

impl ModelName for AccountMoveLineFromOdoo18 {
    const NAME: &'static str = ACCOUNT_MOVE_LINE_MODEL_NAME;
}

#[derive(Debug, Serialize, FieldNamesAsSlice)]
pub struct AccountMoveLineToOdoo19 {
    // Accounting fields
    pub move_id: u64,

    pub debit: Option<f32>,
    pub credit: Option<f32>,

    pub currency_id: Option<u64>,
    pub partner_id: Option<u64>,

    pub display_type: AccountMoveLineDisplayType,
    pub product_id: Option<u64>,
    pub quantity: f32,
    pub price_unit: f32,
    pub discount: Option<f32>,
    pub account_id: Option<u64>,

    pub is_refund: bool,
}
