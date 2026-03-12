use std::collections::HashMap;

use jsonrpsee::{
    core::{client::ClientT, traits::ToRpcParams},
    http_client::HttpClient,
};
use serde::{Serialize, de::DeserializeOwned};
use struct_field_names_as_array::FieldNamesAsSlice;
use url::Url;

use crate::{error, utils::NumOrVec};

use crate::{
    ModelName,
    utils::{Domain, PaginationParam},
    utils::{fields_get::FieldsGetAttributes, version},
};

#[derive(Debug)]
pub struct OdooJsonRPCClient {
    password: String,
    user: String,
    database: String,
    client: HttpClient,
    uid: Option<u32>,
}

#[derive(Debug, Serialize)]
struct OdooJsonRpcBaseArgs<T> {
    service: String,
    method: String,
    args: T,
}

impl<T> ToRpcParams for OdooJsonRpcBaseArgs<T>
where
    T: Serialize,
{
    fn to_rpc_params(self) -> Result<Option<Box<serde_json::value::RawValue>>, serde_json::Error> {
        let val = serde_json::value::to_raw_value(&self)?;
        Ok(Some(val))
    }
}

impl OdooJsonRPCClient {
    pub fn get_uid(&self) -> Option<u32> {
        self.uid
    }
    pub async fn call<P, O>(
        &self,
        service: String,
        method: String,
        args: P,
    ) -> Result<O, error::Error>
    where
        P: Serialize + Send,
        O: DeserializeOwned,
    {
        Ok(self
            .client
            .request(
                "call",
                OdooJsonRpcBaseArgs {
                    service,
                    method,
                    args,
                },
            )
            .await?)
    }
    pub async fn login(&mut self) -> Result<(), error::Error> {
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
    ) -> Result<Self, error::Error> {
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
    pub async fn version(&self) -> Result<version::Version, error::Error> {
        self.call("common".into(), "version".into(), Vec::<String>::new())
            .await
    }

    pub async fn execute_0<O>(
        &self,
        model: String,
        method: String,
        additional_args: Vec<serde_json::Value>,
    ) -> Result<O, error::Error>
    where
        O: DeserializeOwned,
    {
        let mut call_args: Vec<serde_json::Value> = vec![
            self.database.clone().into(),
            self.uid.ok_or(error::Error::NotLoggedIn)?.into(),
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
    ) -> Result<Vec<u64>, error::Error> {
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
    ) -> Result<Vec<O>, error::Error>
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
    pub async fn search_read_with_auto_field_names<O>(
        &self,
        model: String,
        domains: Vec<Domain>,
        pagination: PaginationParam,
    ) -> Result<Vec<O>, error::Error>
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
    pub async fn search_read_with_auto_model_name_and_field_names<O>(
        &self,
        domains: Vec<Domain>,
        pagination: PaginationParam,
    ) -> Result<Vec<O>, error::Error>
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
    ) -> Result<u64, error::Error> {
        let args: Vec<serde_json::Value> = vec![serde_json::to_value(domains)?];
        self.execute_0(model, "search_count".into(), args).await
    }
    pub async fn read<O>(
        &self,
        model: String,
        ids: Vec<u64>,
        fields: Vec<String>,
    ) -> Result<Vec<O>, error::Error>
    where
        O: DeserializeOwned,
    {
        let mut additional_args = vec![serde_json::to_value(ids)?];
        if !fields.is_empty() {
            additional_args.push(serde_json::to_value(fields)?);
        }
        self.execute_0(model, "read".into(), additional_args).await
    }
    pub async fn read_with_auto_model_name<O>(
        &self,
        ids: Vec<u64>,
        fields: Vec<String>,
    ) -> Result<Vec<O>, error::Error>
    where
        O: DeserializeOwned + ModelName,
    {
        let mut additional_args = vec![serde_json::to_value(ids)?];
        if !fields.is_empty() {
            additional_args.push(serde_json::to_value(fields)?);
        }
        self.execute_0(O::NAME.into(), "read".into(), additional_args)
            .await
    }
    pub async fn read_with_auto_model_name_and_field_names<O>(
        &self,
        ids: Vec<u64>,
    ) -> Result<Vec<O>, error::Error>
    where
        O: DeserializeOwned + ModelName + FieldNamesAsSlice,
    {
        let mut additional_args = vec![serde_json::to_value(ids)?];
        if !O::FIELD_NAMES_AS_SLICE.is_empty() {
            additional_args.push(serde_json::to_value(O::FIELD_NAMES_AS_SLICE)?);
        }
        self.execute_0(O::NAME.into(), "read".into(), additional_args)
            .await
    }
    pub async fn fields_get(
        &self,
        model: String,
        fields: Vec<String>,
        attributes: Vec<FieldsGetAttributes>,
    ) -> Result<HashMap<String, HashMap<FieldsGetAttributes, String>>, error::Error> {
        let mut additional_args = vec![serde_json::to_value(fields)?];
        if !attributes.is_empty() {
            additional_args.push(serde_json::to_value(attributes)?);
        } else {
            additional_args.push(serde_json::to_value(vec![
                FieldsGetAttributes::String,
                FieldsGetAttributes::Help,
                FieldsGetAttributes::Type,
            ])?);
        }
        self.execute_0(model, "fields_get".into(), additional_args)
            .await
    }
    pub async fn create<T>(&self, model: String, values: Vec<T>) -> Result<Vec<u64>, error::Error>
    where
        T: Serialize,
    {
        let additional_args = vec![serde_json::to_value(values)?];

        let res: NumOrVec<u64> = self
            .execute_0(model, "create".into(), additional_args)
            .await?;
        Ok(res.into())
    }
    pub async fn create_with_auto_module_name<T>(
        &self,
        values: Vec<T>,
    ) -> Result<Vec<u64>, error::Error>
    where
        T: Serialize + ModelName,
    {
        self.create(T::NAME.into(), values).await
    }
}
