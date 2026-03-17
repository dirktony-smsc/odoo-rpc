use derive_more::derive::Display;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum SaleOrderState {
    #[display("Quotation")]
    Draft,
    #[display("Quotation Sent")]
    Sent,
    #[display("Sales Order")]
    Sale,
    #[display("Cancelled")]
    Cancel,
}

pub const SALES_ORDER_MODEL_NAME: &str = "sale.order";

#[derive(Debug, Clone, Deserialize, FieldNamesAsSlice)]
pub struct SalesOrderFrom18 {
    pub id: u64,
    pub active: bool,

    pub name: String,

    pub company_id: Many2OneRepr,
    pub partener_id: Many2OneRepr,
    pub state: SaleOrderState,
    pub locked: bool,

    pub client_order_ref: String,
    pub create_date: String,
    pub commitment_date: String,
    pub date_order: String,
    pub origin: String,
    pub reference: String,

    pub partner_invoice_id: Many2OneRepr,
    pub partner_shipping_id: Many2OneRepr,
}

impl ModelName for SalesOrderFrom18 {
    const NAME: &'static str = SALES_ORDER_MODEL_NAME;
}
