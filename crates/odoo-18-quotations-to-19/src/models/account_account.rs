use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

pub const ACCOUNT_ACCOUNT_MODEL_NAME: &str = "account.account";

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountAccountType {
    #[display("Receivable")]
    AssetReceivable,
    #[display("Bank and Cash")]
    AssetCash,
    #[display("Current Assets")]
    AssetCurrent,
    #[display("Non-current Assets")]
    AssetNonCurrent,
    #[display("Prepayments")]
    AssetPrepayments,
    #[display("Fixed Assets")]
    AssetFixed,
    #[display("Payable")]
    LiabilityPayable,
    #[display("Credit Card")]
    LiabilityCreditCard,
    #[display("Current Liabilities")]
    LiabilityCurrent,
    #[display("Non-current Liabilities")]
    LiabilityNonCurrent,
    #[display("Equity")]
    Equity,
    #[display("Current Year Earnings")]
    EquityUnaffected,
    #[display("Income")]
    Income,
    #[display("Other Income")]
    IncomeOther,
    #[display("Expenses")]
    Expense,
    #[display("Depreciation")]
    ExpenseDepreciation,
    #[display("Cost of Revenue")]
    ExpenseDirectCost,
    #[display("Off-Balance Sheet")]
    OffBalance,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountAccountInternalGroup {
    #[display("Equity")]
    Equity,
    #[display("Asset")]
    Asset,
    #[display("Liability")]
    Liability,
    #[display("Income")]
    Income,
    #[display("Expense")]
    Expense,
    #[display("Off Balance")]
    Off,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice, Serialize)]
pub struct AccountAccountFromOdoo18 {
    pub id: u64,

    pub name: String,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub currency_id: Option<Many2OneRepr>,
    pub code: String,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub code_store: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub placeholder_code: Option<String>,
    pub deprecated: bool,
    pub used: bool,
    pub account_type: AccountAccountType,
    pub include_initial_balance: bool,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub internal_group: Option<AccountAccountInternalGroup>,
    pub reconcile: bool,
    pub tax_ids: Vec<u64>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub note: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub opening_debit: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub opening_credit: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub opening_balance: Option<f32>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub current_balance: Option<f32>,
    pub related_taxes_amount: u64,

    pub non_trade: bool,

    pub display_mapping_tab: bool,
}

// pub fn account_journal_from_odoo_18_fields() -> Vec<String> {
//     let mut d = AccountJournalFromOdoo18::FIELD_NAMES_AS_SLICE
//         .iter()
//         .map(|d| String::from(*d))
//         .collect::<Vec<_>>();
//     d.push("type".into());
//     d
// }

impl ModelName for AccountAccountFromOdoo18 {
    const NAME: &'static str = ACCOUNT_ACCOUNT_MODEL_NAME;
}

#[derive(Debug, Serialize, FieldNamesAsSlice)]
pub struct AccountAccountToOdoo19 {
    pub name: String,
    pub currency_id: Option<u64>,
    pub code: String,
    pub code_store: Option<String>,
    pub placeholder_code: Option<String>,
    pub deprecated: bool,
    pub used: bool,
    pub account_type: AccountAccountType,
    pub include_initial_balance: bool,
    pub internal_group: Option<AccountAccountInternalGroup>,
    pub reconcile: bool,
    pub tax_ids: Vec<u64>,
    pub note: Option<String>,
    pub opening_debit: Option<f32>,
    pub opening_credit: Option<f32>,
    pub opening_balance: Option<f32>,

    pub current_balance: Option<f32>,
    pub related_taxes_amount: u64,

    pub non_trade: bool,

    pub display_mapping_tab: bool,
}
