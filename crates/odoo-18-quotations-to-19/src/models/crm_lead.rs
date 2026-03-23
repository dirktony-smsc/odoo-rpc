use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::{Many2OneRepr, crm_stage::CrmStagePriorities};

pub const CRM_LEAD_MODEL_NAME: &str = "crm.lead";

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum TypeCrmLead {
    #[display("Lead")]
    Lead,
    #[display("Opportunity")]
    Opportunity,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice)]
pub struct CrmLeadFromOdoo18 {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    #[field_names_as_slice(skip)]
    pub type_: TypeCrmLead,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub referred: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub description: Option<String>,
    pub active: bool,

    pub priority: CrmStagePriorities,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub stage_id: Option<Many2OneRepr>,
    pub color: u16,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub expected_revenue: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub prorated_revenue: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub recurring_revenue: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub recurring_revenue_monthly: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub recurring_revenue_monthly_prorated: Option<f32>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub recurring_revenue_prorated: Option<f32>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub date_closed: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub date_automation_last: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub date_open: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub date_conversion: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub date_deadline: Option<String>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub partner_id: Option<Many2OneRepr>,
}

impl ModelName for CrmLeadFromOdoo18 {
    const NAME: &'static str = CRM_LEAD_MODEL_NAME;
}

pub fn crm_lead_from_odoo_18_fields() -> Vec<String> {
    let mut d = CrmLeadFromOdoo18::FIELD_NAMES_AS_SLICE
        .iter()
        .map(|d| String::from(*d))
        .collect::<Vec<_>>();
    d.push("type".into());
    d
}
