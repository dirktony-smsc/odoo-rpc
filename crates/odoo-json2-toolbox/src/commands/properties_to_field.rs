use std::{collections::HashMap, str::FromStr};

use clap::Args;
use odoo_api_commons::PaginationParam;
use odoo_json2::{
    OdooJson2Client,
    base_methods::{read::ReadParam, search_read::SearchReadParam, write::WriteParam},
};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;

const SOURCE_PATTERN: &str = r"(?<field>\w+):(?<property_id>\w+)";

#[derive(Debug, Deserialize)]
struct PropertyRepr {
    name: String,
    value: Option<serde_json::Value>,
}

#[derive(Debug, Args)]
pub struct PropertiesToFieldArg {
    /// Fetch limit per batch
    ///
    /// Default: 30
    #[arg(short)]
    limit: Option<u32>,
    /// The model to update.
    model: String,
    /// The `field:property_id` to get the data from.
    source: String,
    /// The target field to set the data to.
    target: String,
    /// The model row id to update.
    ///
    /// If not set, all row will be updated.
    #[arg(short, long = "id")]
    ids: Vec<u64>,
}

pub async fn properties_to_field(
    client: &OdooJson2Client,
    arg: PropertiesToFieldArg,
) -> anyhow::Result<()> {
    let total = {
        if arg.ids.is_empty() {
            client
                .search_count(arg.model.clone(), Default::default())
                .await?
        } else {
            arg.ids.len().try_into()?
        }
    };
    log::info!("updating {} values", total);
    let mut offset: u64 = 0;
    let limit = arg.limit.unwrap_or(30);
    let source = arg.source.parse::<SourceField>()?;
    while let Some(batch) = next_batch_data(
        client,
        &mut offset,
        limit,
        total,
        &source,
        &arg.model,
        &arg.ids,
    )
    .await?
    {
        for (id, val) in batch {
            log::trace!("writing {:?} to {}:{}", val.value, id, arg.source);
            client
                .write(
                    arg.model.clone(),
                    WriteParam {
                        ids: vec![id],
                        vals: json!({
                            (&arg.target): &val.value
                        }),
                    },
                )
                .await?;
        }
    }
    Ok(())
}

async fn next_batch_data(
    client: &OdooJson2Client,
    offset: &mut u64,
    limit: u32,
    total: u64,
    source: &SourceField,
    model: &str,
    ids: &[u64],
) -> anyhow::Result<Option<Vec<(u64, PropertyRepr)>>> {
    if *offset > total {
        return Ok(None);
    }
    let values: Vec<HashMap<String, serde_json::Value>> = if ids.is_empty() {
        client
            .search_read(
                model.into(),
                SearchReadParam {
                    fields: vec![source.field.clone()],
                    pagination: Some(PaginationParam {
                        limit: Some(limit),
                        offset: Some((*offset).try_into()?),
                    }),
                    ..Default::default()
                },
            )
            .await?
    } else {
        let ids = ids
            .iter()
            .skip((*offset).try_into()?)
            .take(limit.try_into()?)
            .copied()
            .collect::<Vec<_>>();
        client
            .read(
                model.into(),
                ReadParam {
                    ids,
                    fields: vec![source.field.clone()],
                },
            )
            .await?
    };
    let res = values
        .into_iter()
        .map(|val| -> anyhow::Result<(u64, PropertyRepr)> {
            let id = val
                .get("id")
                .and_then(|v| v.as_u64())
                .ok_or(anyhow::anyhow!("Cannot get id"))?;
            let value = Vec::<PropertyRepr>::deserialize(
                val.get(&source.field)
                    .ok_or(anyhow::anyhow!("No field named `{}`", source.field))?,
            )?;
            let value = value
                .into_iter()
                .find(|val| val.name == source.property_id)
                .ok_or(anyhow::anyhow!("No property named {}", source.property_id))?;
            Ok((id, value))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    *offset += limit as u64;
    Ok(Some(res))
}

fn get_source_partern_regex() -> Result<Regex, regex::Error> {
    Regex::new(SOURCE_PATTERN)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseSourceTargetFieldEror {
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error("Missing field argument")]
    MissingField,
    #[error("Missing property_id argument")]
    MissingPropertyId,
    #[error("The input string doesn't provide any match to the regex")]
    NoMatch,
}

#[derive(Debug)]
struct SourceField {
    field: String,
    property_id: String,
}

impl FromStr for SourceField {
    type Err = ParseSourceTargetFieldEror;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = get_source_partern_regex()?;
        let captures = regex
            .captures(s)
            .ok_or(ParseSourceTargetFieldEror::NoMatch)?;
        Ok(Self {
            field: captures
                .name("field")
                .ok_or(ParseSourceTargetFieldEror::MissingField)?
                .as_str()
                .into(),
            property_id: captures
                .name("property_id")
                .ok_or(ParseSourceTargetFieldEror::MissingPropertyId)?
                .as_str()
                .into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_pattern_regex() {
        let _ = get_source_partern_regex().unwrap();
    }

    #[test]
    fn test_source_parsing() {
        let source = "properties:1334324".parse::<SourceField>().unwrap();
        assert_eq!(source.field, "properties");
        assert_eq!(source.property_id, "1334324");
    }
}
