pub mod version;

use jsonrpsee::{
    core::{client::ClientT, traits::ToRpcParams},
    http_client::HttpClient,
};
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

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
    pub async fn execute<P, O>(
        &self,
        model: String,
        method: String,
        args: P,
    ) -> Result<O, crate::error::Error>
    where
        P: Serialize + Send,
        O: DeserializeOwned,
    {
        let call_args: Vec<serde_json::Value> = vec![
            self.database.clone().into(),
            self.uid.ok_or(crate::error::Error::NotLoggedIn)?.into(),
            self.password.clone().into(),
            model.into(),
            method.into(),
            serde_json::to_value(args)?,
        ];
        self.call("object".into(), "execute".into(), call_args)
            .await
    }
    pub async fn execute_1<P1, P2, O>(
        &self,
        model: String,
        method: String,
        args0: P1,
        args1: P2,
    ) -> Result<O, crate::error::Error>
    where
        P1: Serialize + Send,
        P2: Serialize + Send,
        O: DeserializeOwned,
    {
        let call_args: Vec<serde_json::Value> = vec![
            self.database.clone().into(),
            self.uid.ok_or(crate::error::Error::NotLoggedIn)?.into(),
            self.password.clone().into(),
            model.into(),
            method.into(),
            serde_json::to_value(args0)?,
            serde_json::to_value(args1)?,
        ];
        self.call("object".into(), "execute".into(), call_args)
            .await
    }
}
