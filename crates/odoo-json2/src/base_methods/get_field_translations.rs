use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct GetFieldTranslationsParam {
    pub ids: Vec<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    pub field_name: String,
    pub langs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Translation {
    pub lang: String,
    pub source: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Context {
    pub translation_type: String,
    pub translation_show_source: bool,
}

#[derive(Deserialize)]
struct GetFieldTranslationsOutInner(Vec<Translation>, Context);

impl From<GetFieldTranslationsOutInner> for GetFieldTranslationsOut {
    fn from(value: GetFieldTranslationsOutInner) -> Self {
        Self {
            translations: value.0,
            context: value.1,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "GetFieldTranslationsOutInner")]
pub struct GetFieldTranslationsOut {
    pub translations: Vec<Translation>,
    pub context: Context,
}
