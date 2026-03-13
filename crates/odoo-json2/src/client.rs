use std::sync::{LazyLock, OnceLock};

use reqwest::{
    Client, ClientBuilder, Method, Request, RequestBuilder,
    header::{CONTENT_TYPE, HOST, HeaderValue, USER_AGENT},
};
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::error;

static DEFAULT_USER_AGENT: LazyLock<String> =
    LazyLock::new(|| format!("odoo-json2-rs/{}", env!("CARGO_PKG_VERSION")));

#[derive(Debug, Default)]
#[must_use]
pub struct OdooJson2ClientBuilder {
    rq_cli_builder: ClientBuilder,
    url: Option<Url>,
    database: Option<String>,
    api_key: Option<String>,
    user_agent: Option<String>,
    host: Option<String>,
}

impl OdooJson2ClientBuilder {
    pub fn build(self) -> Result<OdooJson2Client, error::Error> {
        Ok(OdooJson2Client {
            rq_client: self.rq_cli_builder.build()?,
            url: self.url.ok_or(error::Error::BaseUrlRequired)?,
            database: self.database,
            api_key: self.api_key,
            user_agent: self.user_agent,
            host: self.host,
        })
    }
    pub fn reqwest_client_builder(mut self, client_builder: ClientBuilder) -> Self {
        self.rq_cli_builder = client_builder;
        self
    }
    /// Note that the client extract the host from the [`Url::host`] if the [`Self::host`] is not set.
    pub fn base_url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }
    pub fn database(mut self, database: String) -> Self {
        self.database = Some(database.trim().into()).filter(|d: &String| !d.is_empty());
        self
    }
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host.trim().into()).filter(|d: &String| !d.is_empty());
        self
    }
    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent.trim().into()).filter(|d: &String| !d.is_empty());
        self
    }
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key).filter(|d| !d.is_empty());
        self
    }
    pub fn new(base_url: Url) -> Self {
        Self::default().base_url(base_url)
    }
}

pub struct OdooJson2Client {
    rq_client: Client,
    url: Url,
    database: Option<String>,
    api_key: Option<String>,
    user_agent: Option<String>,
    host: Option<String>,
}

impl OdooJson2Client {
    pub fn builder() -> OdooJson2ClientBuilder {
        OdooJson2ClientBuilder::default()
    }
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }
    pub fn get_reqwest_client(&self) -> Client {
        self.rq_client.clone()
    }
    fn make_basic_request(
        &self,
        method: Method,
        path: &str,
    ) -> Result<RequestBuilder, error::Error> {
        let mut req = self.rq_client.request(method, self.url.join(path)?);
        req = req.header(
            HOST,
            if let Some(h) = self.host.as_deref() {
                h
            } else {
                self.url
                    .host_str()
                    .ok_or(error::Error::BaseUrlMissingHost)?
            },
        );

        req = req.header(
            USER_AGENT,
            self.user_agent.as_deref().unwrap_or(&DEFAULT_USER_AGENT),
        );
        Ok(req)
    }

    /// REF: https://www.odoo.com/documentation/19.0/developer/reference/external_api.html#common-service
    pub async fn version(&self) -> Result<crate::version::OdooVersion, crate::error::Error> {
        Ok(self
            .make_basic_request(Method::GET, "/web/version")?
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
    pub async fn call_model_method<I, O>(
        &self,
        model: &str,
        method: &str,
        input: I,
    ) -> Result<O, crate::error::Error>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let mut req_build = self
            .make_basic_request(Method::POST, &format!("/json/2/{model}/{method}"))?
            .bearer_auth(self.api_key.as_ref().ok_or(error::Error::MissingApiKey)?);
        if let Some(db) = self.database.as_ref() {
            req_build = req_build.header("x-odoo-database", db);
        }
        let res = req_build.json(&input).send().await?;
        if res.status().is_client_error() || res.status().is_server_error() {
            match res
                .headers()
                .get(CONTENT_TYPE)
                .as_ref()
                .and_then(|v| v.to_str().ok())
            {
                Some("application/json") => {
                    let er: error::ModelMethodCallError = res.json().await?;
                    Err(er.into())
                }
                _ => {
                    let status = res.status().as_u16();
                    Err(error::Error::AbstractRequest(status, res.text().await?))
                }
            }
        } else {
            Ok(res.json().await?)
        }
    }
}
