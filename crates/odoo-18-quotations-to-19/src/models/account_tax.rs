use derive_more::derive::Display;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

pub const ACCOUNT_TAX_MODEL_NAME: &str = "account.tax";

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum TypeTaxUse {
    #[display("Sales")]
    Sale,
    #[display("Purchases")]
    Purchase,
    #[display("None")]
    None,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice)]
pub struct AccountTax1 {
    pub id: u64,
    pub amount: f32,
    pub type_tax_use: TypeTaxUse,
}

impl ModelName for AccountTax1 {
    const NAME: &'static str = ACCOUNT_TAX_MODEL_NAME;
}
