use derive_more::derive::Display;
use odoo_api_commons::deserialize_and_default_if_false;
use odoo_rpc::ModelName;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::models::Many2OneRepr;

pub const HR_EMPLOYEE_MODEL_NAME: &str = "hr.employee";

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
pub enum HrPresenceState {
    #[display("Present")]
    Present,
    #[display("Absent")]
    Absent,
    #[display("Archived")]
    Archive,
    #[default]
    #[display("Off-Hours")]
    OutOfWorkingHour,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum HrIconDisplay {
    #[display("Present")]
    PresencePresent,
    #[display("Off-Hours")]
    PresenceOutOfWorkingHour,
    #[display("Absent")]
    PresenceAbsent,
    #[display("Archived")]
    PresenceArchive,
    #[display("Undetermined")]
    PresenceUndetermined,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum WorkLocationType {
    Home,
    Office,
    Other,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice, Serialize)]
pub struct HrEmployeeFromOdoo18 {
    pub id: u64,

    // Versions
    pub version_id: Many2OneRepr,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub current_version_id: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub current_date_version: Option<String>,
    pub version_ids: Vec<u64>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub versions_count: u64,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub hr_presence_state: Option<HrPresenceState>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub hr_icon_display: Option<HrIconDisplay>,
    pub show_hr_icon_display: bool,
    pub newly_hired: bool,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub mobile_phone: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub work_phone: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub work_email: Option<String>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub legal_name: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub private_phone: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub private_email: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub lang: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub place_of_birth: Option<Many2OneRepr>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub birthday: Option<String>,
    pub birthday_public_display: bool,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub birthday_public_display_string: Option<String>,

    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub permit_no: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub visa_no: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub visa_expire: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub work_permit_expiration_date: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub has_work_permit: Option<String>,
    pub work_permit_scheduled_activity: bool,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub work_permit_name: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub certificate: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub study_field: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub study_school: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub emergency_contact: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub emergency_phone: Option<String>,
    #[serde(deserialize_with = "deserialize_and_default_if_false")]
    pub work_location_name: Option<String>,
    pub work_location_type: Option<WorkLocationType>,
}

impl ModelName for HrEmployeeFromOdoo18 {
    const NAME: &'static str = HR_EMPLOYEE_MODEL_NAME;
}

#[derive(Debug, Serialize, FieldNamesAsSlice)]
pub struct HrEmployeeToOdoo19 {
    // Versions
    pub version_id: u64,
    pub current_version_id: Option<u64>,
    pub current_date_version: Option<String>,
    pub version_ids: Vec<u64>,
    pub versions_count: u64,

    pub hr_presence_state: Option<HrPresenceState>,
    pub hr_icon_display: Option<HrIconDisplay>,
    pub show_hr_icon_display: bool,
    pub newly_hired: bool,

    pub mobile_phone: Option<String>,
    pub work_phone: Option<String>,
    pub work_email: Option<String>,

    pub legal_name: Option<String>,
    pub private_phone: Option<String>,
    pub private_email: Option<String>,
    pub lang: Option<String>,
    pub place_of_birth: Option<u64>,
    pub birthday: Option<String>,
    pub birthday_public_display: bool,
    pub birthday_public_display_string: Option<String>,

    pub permit_no: Option<String>,
    pub visa_no: Option<String>,
    pub visa_expire: Option<String>,
    pub work_permit_expiration_date: Option<String>,
    pub has_work_permit: Option<String>,
    pub work_permit_scheduled_activity: bool,
    pub work_permit_name: Option<String>,
    pub certificate: Option<String>,
    pub study_field: Option<String>,
    pub study_school: Option<String>,
    pub emergency_contact: Option<String>,
    pub emergency_phone: Option<String>,
    pub work_location_name: Option<String>,
    pub work_location_type: Option<WorkLocationType>,
}
