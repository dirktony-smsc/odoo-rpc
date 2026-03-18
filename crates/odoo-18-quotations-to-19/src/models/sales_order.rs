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

    pub name: String,

    pub company_id: Many2OneRepr,
    pub partner_id: Many2OneRepr,
    pub state: SaleOrderState,
    pub locked: bool,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub client_order_ref: Option<String>,
    pub create_date: String,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub commitment_date: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub date_order: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub origin: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub reference: Option<String>,

    pub partner_invoice_id: Many2OneRepr,
    pub partner_shipping_id: Many2OneRepr,
}

impl ModelName for SalesOrderFrom18 {
    const NAME: &'static str = SALES_ORDER_MODEL_NAME;
}

#[derive(Debug, Serialize)]
pub struct SalesOrderTo19 {
    pub name: String,
    pub state: SaleOrderState,
    pub partner_id: u64,
    pub client_order_ref: Option<String>,
    pub create_date: String,

    pub commitment_date: Option<String>,
    pub date_order: Option<String>,
    pub origin: Option<String>,
    pub reference: Option<String>,

    pub partner_invoice_id: u64,
    pub partner_shipping_id: u64,
}

impl From<SalesOrderFrom18> for SalesOrderTo19 {
    fn from(value: SalesOrderFrom18) -> Self {
        SalesOrderTo19 {
            name: value.name,
            state: value.state,
            partner_id: value.partner_id.id,
            client_order_ref: value.client_order_ref,
            create_date: value.create_date,
            commitment_date: value.commitment_date,
            date_order: value.date_order,
            origin: value.origin,
            reference: value.reference,
            partner_invoice_id: value.partner_invoice_id.id,
            partner_shipping_id: value.partner_shipping_id.id,
        }
    }
}
