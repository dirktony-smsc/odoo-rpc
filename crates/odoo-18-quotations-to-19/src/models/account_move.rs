use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

pub const ACCOUNT_MOVE_MODEL_NAME: &str = "account.move";

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum PayementStateSelection {
    #[display("Not Paid")]
    NotPaid,
    #[display("In Payment")]
    InPayment,
    #[display("Paid")]
    Paid,
    #[display("Partially Paid")]
    Partial,
    #[display("Reversed")]
    Reversed,
    #[display("Blocked")]
    Blocked,
    #[display("Invoicing App Legacy")]
    InvoicingLegacy,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountMoveState {
    #[display("Draft")]
    Draft,
    #[display("Posted")]
    Posted,
    #[display("Cancelled")]
    Cancel,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountMoveType {
    #[display("Draft")]
    Entry,
    #[display("Customer Invoice")]
    OutInvoice,
    #[display("Customer Credit Note")]
    OutRefund,
    #[display("Vendor Bill")]
    InInvoice,
    #[display("Vendor Credit Note")]
    InRefund,
    #[display("Sales Receipt")]
    OutReceipt,
    #[display("Purchase Receipt")]
    InReceipt,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AutoPostAccountMove {
    #[display("No")]
    No,
    #[display("At Date")]
    AtDate,
    #[display("Monthly")]
    Monthly,
    #[display("Quarterly")]
    Quarterly,
    #[display("Yearly")]
    Yearly,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice)]
pub struct AccountMoveFromOdoo18 {
    pub id: u64,

    // Accounting fields
    pub name: String,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub name_placeholder: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false", rename = "ref")]
    #[field_names_as_slice(skip)]
    pub ref_: Option<String>,
    pub date: String,
    pub state: AccountMoveState,
    pub move_type: AccountMoveType,
    pub is_storno: bool,
    pub journal_id: Many2OneRepr,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub journal_group_id: Option<Many2OneRepr>,

    pub auto_post: AutoPostAccountMove,
    pub currency_id: Many2OneRepr,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub partner_id: Option<Many2OneRepr>,
}

pub fn account_move_from_odoo_18_fields() -> Vec<String> {
    let mut d = AccountMoveFromOdoo18::FIELD_NAMES_AS_SLICE
        .iter()
        .map(|d| String::from(*d))
        .collect::<Vec<_>>();
    d.push("ref".into());
    d
}

impl ModelName for AccountMoveFromOdoo18 {
    const NAME: &'static str = ACCOUNT_MOVE_MODEL_NAME;
}

#[derive(Debug, Serialize, FieldNamesAsSlice)]
pub struct AccountMoveToOdoo19 {
    // Accounting fields
    pub name: String,
    pub name_placeholder: Option<String>,
    #[field_names_as_slice(skip)]
    pub ref_: Option<String>,
    pub date: String,
    pub state: AccountMoveState,
    pub move_type: AccountMoveType,
    pub is_storno: bool,
    pub journal_id: u64,

    pub auto_post: AutoPostAccountMove,
    pub currency_id: u64,

    pub partner_id: Option<u64>,
}
