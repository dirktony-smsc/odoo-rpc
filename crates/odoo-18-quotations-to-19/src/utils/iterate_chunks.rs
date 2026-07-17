use std::num::NonZero;

use log::{debug, trace, warn};
use odoo_api_commons::{Domain, PaginationParam};
use odoo_rpc::OdooJsonRPCClient;
use serde::de::DeserializeOwned;

use crate::error;

pub struct IterateModelFromOdoo18<'a> {
    per_chunck: NonZero<u32>,
    offset: u32,
    domains: Vec<Domain>,
    count: u64,
    model_name: String,
    field_names: Vec<String>,
    client: &'a OdooJsonRPCClient,
}

impl<'a> IterateModelFromOdoo18<'a> {
    pub async fn new(
        client: &'a OdooJsonRPCClient,
        model_name: String,
        field_names: Vec<String>,
        domains: Vec<Domain>,
        per_chunck: NonZero<u32>,
        offset: u32,
    ) -> Result<Self, error::Error> {
        let count = client
            .search_count(model_name.clone(), domains.clone())
            .await?;
        Ok(Self {
            per_chunck,
            offset,
            domains,
            count,
            model_name,
            field_names,
            client,
        })
    }
    fn get_current_pagination(&self) -> PaginationParam {
        PaginationParam {
            offset: Some(self.offset),
            limit: Some(self.per_chunck.get()),
        }
    }
    pub async fn next<O: DeserializeOwned>(&mut self) -> Option<Result<Vec<O>, error::Error>> {
        if (self.offset as u64) > self.count {
            trace!("Going out of bound...");
            return None;
        }
        debug!(
            "current offset = {} ; total count = {}",
            self.offset, self.count
        );
        let res = match self
            .client
            .search_read::<O>(
                self.model_name.clone(),
                self.domains.clone(),
                self.field_names.clone(),
                self.get_current_pagination(),
            )
            .await
        {
            Ok(a) => a,
            Err(err) => return Some(Err(err.into())),
        };
        let next_offset = self.offset + self.per_chunck.get();
        self.offset = next_offset;
        Some(Ok(res))
    }
    pub fn count(&self) -> u64 {
        self.count
    }
}
