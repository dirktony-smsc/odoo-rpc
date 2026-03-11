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
    uid: String,
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
    pub fn get_uid(&self) -> Option<&str> {
        if self.uid.is_empty() {
            None
        } else {
            Some(&self.uid)
        }
    }
    pub async fn call<P: Serialize + Send, O: DeserializeOwned>(
        &self,
        service: String,
        method: String,
        args: P,
    ) -> Result<O, crate::error::Error> {
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
        let uid: String = self
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
        self.uid = uid;
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
}
