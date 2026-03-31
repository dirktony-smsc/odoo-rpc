use anyhow::{Ok, anyhow};
use clap::{Args, ValueEnum};
use log::{debug, trace, warn};
use odoo_api_commons::{PaginationParam, command::CommandRepr};
use odoo_json2::{
    OdooJson2Client,
    base_methods::{field_get::FieldsGetParam, search::SearchParam, write::WriteParam},
};
use odoo_rpc::{OdooJsonRPCClient, utils::fields_get::FieldsGetAttributes};
use serde::Deserialize;
use serde_json::json;

use crate::client::Clients;

#[derive(Debug, ValueEnum, Default, Clone, Copy)]
pub enum BatchUpdateOdooVersion {
    #[default]
    Odoo19,
    Odoo18,
}

#[derive(Debug, Args)]
pub struct BatchUpdateArg {
    model: String,
    field: String,
    value: String,
    #[arg(long)]
    ids: Vec<u64>,
    #[arg(long, short)]
    limit: Option<u32>,
    #[arg(long)]
    odoo_version: Option<BatchUpdateOdooVersion>,
}

impl BatchUpdateArg {
    fn limit(&self) -> u32 {
        self.limit.unwrap_or(30)
    }
    pub async fn run(self, client: &Clients) -> anyhow::Result<()> {
        match self.odoo_version.unwrap_or_default() {
            BatchUpdateOdooVersion::Odoo19 => self.run_throught_odoo_19(&client.odoo_19).await,
            BatchUpdateOdooVersion::Odoo18 => self.run_throught_odoo_18(&client.odoo_18).await,
        }
    }
    async fn run_throught_odoo_19(self, client: &OdooJson2Client) -> anyhow::Result<()> {
        let value: serde_json::Value = {
            let res = client
                .fields_get::<FieldGetEntity>(
                    self.model.clone(),
                    FieldsGetParam {
                        allfields: vec![self.field.clone()],
                        attributes: Some(vec!["type".into()]),
                        ..Default::default()
                    },
                )
                .await?;
            trace!("{:#?}", res);
            match res
                .get(&self.field)
                .ok_or(anyhow!("Cannot find {} field", self.field))?
                .type_
                .as_str()
            {
                "integer" | "float" | "monetary" | "many2one" => {
                    serde_json::Value::Number(self.value.parse::<serde_json::Number>()?)
                }
                "char" | "text" | "date" | "selection" | "html" => {
                    serde_json::Value::String(self.value.clone())
                }
                "one2many" | "many2many" => {
                    serde_json::to_value(serde_json::from_str::<Vec<CommandRepr>>(&self.value)?)?
                }
                s => return Err(anyhow!("type {s} for {} is not supported", self.field)),
            }
        };
        if self.ids.is_empty() {
            warn!("No ids given. Updating everything...");
            let count = client
                .search_count(self.model.clone(), Default::default())
                .await?;
            let mut offset = 0u32;
            let limit = self.limit();
            loop {
                debug!("Fetching ({offset} - {limit}) of {count}");
                let ids = client
                    .search(
                        self.model.clone(),
                        SearchParam {
                            pagination: Some(PaginationParam {
                                offset: Some(offset),
                                limit: Some(limit),
                            }),
                            ..Default::default()
                        },
                    )
                    .await?;
                if !ids.is_empty() {
                    client
                        .write(
                            self.model.clone(),
                            WriteParam {
                                ids,
                                vals: json!({
                                    self.field.clone(): value.clone()
                                }),
                            },
                        )
                        .await?;
                }
                {
                    let next_offset = offset + limit;
                    if (next_offset as u64) > count {
                        break;
                    } else {
                        offset = next_offset;
                    }
                }
            }
        } else {
            client
                .write(
                    self.model,
                    WriteParam {
                        ids: self.ids,
                        vals: json!({
                            self.field.clone(): value.clone()
                        }),
                    },
                )
                .await?;
        }
        Ok(())
    }
    async fn run_throught_odoo_18(self, client: &OdooJsonRPCClient) -> anyhow::Result<()> {
        let value: serde_json::Value = {
            let field_get = client
                .fields_get(
                    self.model.clone(),
                    vec![self.field.clone()],
                    vec![FieldsGetAttributes::Type],
                )
                .await?;
            match field_get
                .get(&self.field)
                .and_then(|o| {
                    o.get(&odoo_rpc::utils::fields_get::FieldsGetAttributes::Type)
                        .cloned()
                })
                .ok_or(anyhow!("Cannot find {} field", self.field))?
                .as_str()
            {
                "integer" | "float" | "monetary" | "many2one" => {
                    serde_json::Value::Number(self.value.parse::<serde_json::Number>()?)
                }
                "char" | "text" | "date" | "selection" | "html" => {
                    serde_json::Value::String(self.value.clone())
                }
                "one2many" | "many2many" => {
                    serde_json::to_value(serde_json::from_str::<Vec<CommandRepr>>(&self.value)?)?
                }
                s => return Err(anyhow!("type {s} for {} is not supported", self.field)),
            }
        };
        if self.ids.is_empty() {
            warn!("No ids given. Updating everything...");
            let count = client
                .search_count(self.model.clone(), Default::default())
                .await?;
            let mut offset = 0u32;
            let limit = self.limit();
            loop {
                debug!("Fetching ({offset} - {limit}) of {count}");
                let ids = client
                    .search(
                        self.model.clone(),
                        Default::default(),
                        PaginationParam {
                            offset: Some(offset),
                            limit: Some(limit),
                        },
                    )
                    .await?;
                for id in &ids {
                    client
                        .update(
                            self.model.clone(),
                            *id,
                            json!({
                                self.field.clone(): value.clone()
                            }),
                        )
                        .await?;
                }
                {
                    let next_offset = offset + limit;
                    if (next_offset as u64) > count {
                        break;
                    } else {
                        offset = next_offset;
                    }
                }
            }
        } else {
            for id in &self.ids {
                client
                    .update(
                        self.model.clone(),
                        *id,
                        json!({
                            self.field.clone(): value.clone()
                        }),
                    )
                    .await?;
            }
        }
        todo!()
    }
}

#[derive(Debug, Deserialize)]
struct FieldGetEntity {
    #[serde(rename = "type")]
    type_: String,
}
