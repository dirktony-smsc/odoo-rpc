use std::{collections::HashMap, num::NonZero, str::FromStr};

use clap::Args;
use log::{debug, info, trace};
use odoo_api_commons::PaginationParam;
use odoo_json2::{
    OdooJson2Client,
    base_methods::{search_read::SearchReadParam, write::WriteParam},
};
use serde::Deserialize;
use serde_json::json;

use crate::{client::Clients, utils::partner_cache::get_mapping_partner_o18_to_o19};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X19PropertyName {
    /// The property name
    Name(String),
    /// The property string value
    String(String),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum X19PropertyNameParseError {
    Regex(#[from] regex::Error),
    #[error("The `inner` of the `string:()` is empty")]
    EmptyInner,
}

impl FromStr for X19PropertyName {
    type Err = X19PropertyNameParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgx = regex::Regex::new(r"string:\((?<inner>\w+)\)")?;
        let Some(captures) = rgx.captures(s) else {
            return Ok(Self::Name(s.to_string()));
        };
        Ok(Self::String(
            captures
                .name("inner")
                .ok_or(X19PropertyNameParseError::EmptyInner)?
                .as_str()
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod x19_props_name_tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(
            "123ssdaca".parse::<X19PropertyName>().unwrap(),
            X19PropertyName::Name("123ssdaca".into())
        );
    }
    #[test]
    fn test_string() {
        assert_eq!(
            "string:(allo)".parse::<X19PropertyName>().unwrap(),
            X19PropertyName::String("allo".into())
        );
    }
}

#[derive(Debug, Deserialize)]
struct SimpleO19PartnerRepr {
    properties: Vec<X19Property>,
}

#[derive(Debug, Deserialize)]
struct X19Property {
    name: String,
    string: String,
}

impl X19PropertyName {
    pub async fn get_name(self, client: &OdooJson2Client) -> Result<String, crate::error::Error> {
        match self {
            X19PropertyName::Name(name) => Ok(name),
            X19PropertyName::String(val) => {
                let res = client
                    .search_read::<SimpleO19PartnerRepr>(
                        "res.partner".into(),
                        SearchReadParam {
                            pagination: Some(PaginationParam {
                                limit: Some(1),
                                ..Default::default()
                            }),
                            fields: vec!["properties".into()],
                            ..Default::default()
                        },
                    )
                    .await?;
                Ok(res
                    .first()
                    .ok_or(crate::error::Error::PropertiesNotFound(val.clone()))?
                    .properties
                    .iter()
                    .find(|prop| prop.string == val)
                    .ok_or(crate::error::Error::PropertiesNotFound(val.clone()))?
                    .name
                    .clone())
            }
        }
    }
}

#[derive(Debug, Args)]
pub struct ResPartner18FieldsTo19PropertiesArg {
    /// The Odoo 18 contact (res.partner) field to take the value from
    x_18_field: String,
    /// The Odoo 19 contact (res.parter) "properties" name to put the O18 field value into.
    ///
    /// Note that this is not the name of the field on the UI, it is the "property ID" i should say.
    ///
    /// If you want to use the "string" value, wrap it inside a `string:(<your value here>)`.
    x_19_property_name: X19PropertyName,
    #[arg(long, short)]
    batch_limit: Option<NonZero<u32>>,
}

impl ResPartner18FieldsTo19PropertiesArg {
    pub async fn run(self, clients: &Clients) -> Result<(), crate::error::Error> {
        let Self {
            x_18_field,
            x_19_property_name,
            batch_limit: limit,
        } = self;
        let x_19_property_name = x_19_property_name.get_name(&clients.odoo_19).await?;
        info!("found odoo 19 name id = {x_19_property_name}");
        let limit = limit.map(|d| d.get()).unwrap_or(30);
        let mut offset = 0u32;
        let count = clients
            .odoo_18
            .search_count("res.partner".into(), Default::default())
            .await?;
        loop {
            debug!("Fetching ({offset} - {limit}) of {count}");
            let ids = clients
                .odoo_18
                .search(
                    "res.partner".into(),
                    Default::default(),
                    PaginationParam {
                        offset: Some(offset),
                        limit: Some(limit),
                    },
                )
                .await?;
            let values = {
                trace!("Fetching data for {} ids", ids.len());
                let raw_vals = clients
                    .odoo_18
                    .read::<HashMap<String, serde_json::Value>>(
                        "res.partner".into(),
                        ids,
                        vec![x_18_field.clone()],
                    )
                    .await?;
                raw_vals
                    .into_iter()
                    .flat_map(|entry| -> Option<(u64, serde_json::Value)> {
                        Some((entry.get("id")?.as_u64()?, entry.get(&x_18_field)?.clone()))
                    })
                    .collect::<HashMap<u64, serde_json::Value>>()
            };
            for (id, field_value) in values {
                let id = get_mapping_partner_o18_to_o19(clients, id).await?;
                trace!("Updating Odoo 19 ({id}) res.partner props...");
                clients
                    .odoo_19
                    .write(
                        "res.partner".into(),
                        WriteParam {
                            ids: vec![id],
                            vals: json!({
                                "properties": {
                                    "name": &x_19_property_name,
                                    "value": field_value
                                }
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
        Ok(())
    }
}
