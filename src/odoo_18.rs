pub mod version;

use jsonrpsee::{
    core::{client::ClientT, traits::ToRpcParams},
    http_client::HttpClient,
};
use serde::{Serialize, de::DeserializeOwned};
use struct_field_names_as_array::FieldNamesAsSlice;
use url::Url;

use crate::utils::{Domain, PaginationParam};

#[derive(Debug)]
pub struct Odoo18JsonRPCClient {
    password: String,
    user: String,
    database: String,
    client: HttpClient,
    uid: Option<u32>,
}

#[derive(Debug, Serialize)]
struct Odoo18JsonRpcBaseArgs<T> {
    service: String,
    method: String,
    args: T,
}

impl<T> ToRpcParams for Odoo18JsonRpcBaseArgs<T>
where
    T: Serialize,
{
    fn to_rpc_params(self) -> Result<Option<Box<serde_json::value::RawValue>>, serde_json::Error> {
        let val = serde_json::value::to_raw_value(&self)?;
        Ok(Some(val))
    }
}

impl Odoo18JsonRPCClient {
    pub fn get_uid(&self) -> Option<u32> {
        self.uid
    }
    pub async fn call<P, O>(
        &self,
        service: String,
        method: String,
        args: P,
    ) -> Result<O, crate::error::Error>
    where
        P: Serialize + Send,
        O: DeserializeOwned,
    {
        Ok(self
            .client
            .request(
                "call",
                Odoo18JsonRpcBaseArgs {
                    service,
                    method,
                    args,
                },
            )
            .await?)
    }
    pub async fn login(&mut self) -> Result<(), crate::error::Error> {
        let uid: u32 = self
            .call(
                "common".into(),
                "login".into(),
                vec![
                    self.database.as_str(),
                    self.user.as_str(),
                    self.password.as_str(),
                ],
            )
            .await?;
        self.uid = Some(uid);
        Ok(())
    }
    pub async fn new(
        base: Url,
        user: String,
        password: String,
        database: String,
    ) -> Result<Self, crate::error::Error> {
        let mut a = Self {
            database,
            password,
            uid: Default::default(),
            user,
            client: HttpClient::builder().build(base)?,
        };
        a.login().await?;
        Ok(a)
    }
    pub async fn version(&self) -> Result<version::Version, crate::error::Error> {
        self.call("common".into(), "version".into(), Vec::<String>::new())
            .await
    }

    pub async fn execute_0<O>(
        &self,
        model: String,
        method: String,
        additional_args: Vec<serde_json::Value>,
    ) -> Result<O, crate::error::Error>
    where
        O: DeserializeOwned,
    {
        let mut call_args: Vec<serde_json::Value> = vec![
            self.database.clone().into(),
            self.uid.ok_or(crate::error::Error::NotLoggedIn)?.into(),
            self.password.clone().into(),
            model.into(),
            method.into(),
        ];
        call_args.extend(additional_args);
        self.call("object".into(), "execute".into(), call_args)
            .await
    }
    pub async fn search(
        &self,
        model: String,
        domains: Vec<Domain>,
        pagination: PaginationParam,
    ) -> Result<Vec<u64>, crate::error::Error> {
        let mut args: Vec<serde_json::Value> = vec![serde_json::to_value(domains)?];
        args.push(pagination.offset.unwrap_or_default().into());
        if let Some(limit) = pagination.limit {
            args.push(limit.into());
        }
        self.execute_0(model, "search".into(), args).await
    }
    pub async fn search_read<O>(
        &self,
        model: String,
        domains: Vec<Domain>,
        fields: Vec<String>,
        pagination: PaginationParam,
    ) -> Result<Vec<O>, crate::error::Error>
    where
        O: DeserializeOwned,
    {
        let mut args: Vec<serde_json::Value> = vec![
            serde_json::to_value(domains)?,
            serde_json::to_value(fields)?,
        ];
        args.push(pagination.offset.unwrap_or_default().into());
        if let Some(limit) = pagination.limit {
            args.push(limit.into());
        }
        self.execute_0(model, "search_read".into(), args).await
    }
    pub async fn search_read_typed<O>(
        &self,
        model: String,
        domains: Vec<Domain>,
        pagination: PaginationParam,
    ) -> Result<Vec<O>, crate::error::Error>
    where
        O: DeserializeOwned + FieldNamesAsSlice,
    {
        self.search_read(
            model,
            domains,
            O::FIELD_NAMES_AS_SLICE
                .iter()
                .map(|s| String::from(*s))
                .collect(),
            pagination,
        )
        .await
    }
    pub async fn search_read_typed_1<O>(
        &self,
        domains: Vec<Domain>,
        pagination: PaginationParam,
    ) -> Result<Vec<O>, crate::error::Error>
    where
        O: DeserializeOwned + FieldNamesAsSlice + ModelName,
    {
        self.search_read(
            O::NAME.into(),
            domains,
            O::FIELD_NAMES_AS_SLICE
                .iter()
                .map(|s| String::from(*s))
                .collect(),
            pagination,
        )
        .await
    }
    pub async fn search_count(
        &self,
        model: String,
        domains: Vec<Domain>,
    ) -> Result<u64, crate::error::Error> {
        let args: Vec<serde_json::Value> = vec![serde_json::to_value(domains)?];
        self.execute_0(model, "search_count".into(), args).await
    }
}

pub trait ModelName {
    const NAME: &'static str;
}
