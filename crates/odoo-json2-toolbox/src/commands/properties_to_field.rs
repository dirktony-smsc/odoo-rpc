use std::str::FromStr;

use clap::Args;
use odoo_json2::OdooJson2Client;
use regex::Regex;

const SOURCE_PATTERN: &str = r"(?<field>\w+):(?<property_id>\w+)";

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
    Ok(())
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

#[derive(Debug)]
struct TargetField {
    model: String,
    field: String,
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
