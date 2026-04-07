use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

pub const RESOURCE_RESOURCE_MODEL_NAME: &str = "resource.resource";

#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    Default,
)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    #[default]
    #[display("Human")]
    User,
    #[display("Material")]
    Material,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice, Serialize)]
pub struct ResourceResourceFromOdoo18 {
    pub id: u64,
    pub name: String,
    pub active: bool,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub company_id: Option<Many2OneRepr>,
    pub resource_type: ResourceType,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub user_id: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub avatar_128: Option<String>,
    pub time_efficiency: f32,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub calendar_id: Option<Many2OneRepr>,
    pub tz: String,
}

impl ModelName for ResourceResourceFromOdoo18 {
    const NAME: &'static str = RESOURCE_RESOURCE_MODEL_NAME;
}

#[derive(Debug, Deserialize, FieldNamesAsSlice)]
pub struct ResourceResourceToOdoo19 {
    pub name: String,
    pub active: bool,
    pub company_id: Option<u64>,
    pub resource_type: ResourceType,
    pub user_id: Option<u64>,
    pub avatar_128: Option<String>,
    pub time_efficiency: f32,
    pub calendar_id: Option<u64>,
    pub tz: String,
}
