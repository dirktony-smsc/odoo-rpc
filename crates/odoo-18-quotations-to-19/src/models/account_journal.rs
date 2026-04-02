use derive_more::derive::Display;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

pub const ACCOUNT_JOURNAL_MODEL_NAME: &str = "account.journal";

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountJournalType {
    #[display("Sales")]
    Sale,
    #[display("Purchase")]
    Purchase,
    #[display("Cash")]
    Cash,
    #[display("Bank")]
    Bank,
    #[display("Credit Cash")]
    Credit,
    #[display("Miscellaneous")]
    General,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountJournalInvoiceReferenceType {
    #[display("None")]
    None,
    #[display("Based on Customer")]
    Partner,
    #[display("Based on Invoice")]
    Invoice,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountJournalInvoiceReferenceModel {
    #[display("Odoo")]
    Odoo,
    #[display("European")]
    Euro,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice, Serialize)]
pub struct AccountJournalFromOdoo18 {
    pub id: u64,

    pub name: String,
    pub code: String,
    #[serde(default)]
    pub active: bool,
    #[field_names_as_slice(skip)]
    #[serde(rename = "type")]
    pub type_: AccountJournalType,
    pub autocheck_on_post: bool,
    pub restrict_mode_hash_table: bool,
    pub sequence: u16,

    pub invoice_reference_type: AccountJournalInvoiceReferenceType,
    pub invoice_reference_model: AccountJournalInvoiceReferenceModel,
}

pub fn account_journal_from_odoo_18_fields() -> Vec<String> {
    let mut d = AccountJournalFromOdoo18::FIELD_NAMES_AS_SLICE
        .iter()
        .map(|d| String::from(*d))
        .collect::<Vec<_>>();
    d.push("type".into());
    d
}

impl ModelName for AccountJournalFromOdoo18 {
    const NAME: &'static str = ACCOUNT_JOURNAL_MODEL_NAME;
}

#[derive(Debug, Serialize, FieldNamesAsSlice)]
pub struct AccountJournalToOdoo19 {
    pub name: String,
    pub code: String,
    pub active: bool,
    #[serde(rename = "type")]
    pub type_: AccountJournalType,
    // pub autocheck_on_post: bool,
    pub restrict_mode_hash_table: bool,
    pub sequence: u16,

    pub invoice_reference_type: AccountJournalInvoiceReferenceType,
    pub invoice_reference_model: AccountJournalInvoiceReferenceModel,
}
