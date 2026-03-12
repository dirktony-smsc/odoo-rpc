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
}
