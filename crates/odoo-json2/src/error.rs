use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum Error {
    Reqwest(#[from] reqwest::Error),
    SerdeJson(#[from] serde_json::Error),
    #[error("a base url is required to build a OdooJson2Client")]
    BaseUrlRequired,
    #[error("an API key is required to fulfill your request")]
    MissingApiKey,
    #[error("The client base url doesn't have an host")]
    BaseUrlMissingHost,
    ParseUrl(#[from] url::ParseError),
    ModelMethodCall(Box<ModelMethodCallError>),
    #[error("Got {} ({})", .0, .1)]
    AbstractRequest(u16, String),
}

impl From<ModelMethodCallError> for Error {
    fn from(value: ModelMethodCallError) -> Self {
        Self::ModelMethodCall(Box::new(value))
    }
}

#[derive(Debug, thiserror::Error, Deserialize)]
#[non_exhaustive]
#[error("{}: {}", .name, .message)]
pub struct ModelMethodCallError {
    pub name: String,
    pub message: String,
    pub arguments: (String, u16),
    pub context: HashMap<String, serde_json::Value>,
    pub debug: String,
}
