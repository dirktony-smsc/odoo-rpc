use better_default::Default;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateFieldTranslationsParam {
    pub ids: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    pub field_name: String,
    pub source_lang: String,
    #[default(crate::utils::empty_dict())]
    pub translations: serde_json::Value,
}
