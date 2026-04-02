pub mod account_account_cache;
pub mod account_journal_cache;
pub mod crm_stage_cache;
pub mod partner_cache;
pub mod product;
pub mod tax_cache;

use odoo_json2::{
    OdooJson2Client,
    base_methods::{name_create::NameCreateParam, name_search::NameSearchParam},
};
use regex::Regex;
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::error;

#[allow(unused)]
pub trait FieldnamesAsStringVec {
    fn field_names_as_string_vec() -> Vec<String>;
}

impl<T> FieldnamesAsStringVec for T
where
    T: FieldNamesAsSlice,
{
    fn field_names_as_string_vec() -> Vec<String> {
        <Self as FieldNamesAsSlice>::FIELD_NAMES_AS_SLICE
            .iter()
            .map(|d| String::from(*d))
            .collect()
    }
}

pub async fn get_or_create_by_name(
    client: &OdooJson2Client,
    model: String,
    name: String,
) -> Result<u64, error::Error> {
    let maybe_entry = client
        .name_search(
            model.clone(),
            NameSearchParam {
                name: Some(name.clone()),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await?
        .first()
        .cloned();
    if let Some((id, _)) = maybe_entry {
        Ok(id)
    } else {
        Ok(client
            .name_create(
                model.clone(),
                NameCreateParam {
                    name,
                    ..Default::default()
                },
            )
            .await?
            .0)
    }
}

pub async fn get_or_create_partner_by_name(
    client: &OdooJson2Client,
    name: String,
) -> Result<u64, error::Error> {
    get_or_create_by_name(client, "res.partner".into(), name).await
}

pub async fn get_or_create_currency_by_name(
    client: &OdooJson2Client,
    name: String,
) -> Result<u64, error::Error> {
    get_or_create_by_name(client, "res.currency".into(), name).await
}

pub fn remove_slices_from_string(i: &str) -> Result<String, error::Error> {
    let s: String = Regex::new(r"\[[^)]*\]")?.replace(i, "").into();
    log::debug!("{s}");
    Ok(s)
}

pub fn trim_whitespace_v2(s: &str) -> String {
    // second attempt: only allocate a string
    let mut result = String::with_capacity(s.len());
    s.split_whitespace().for_each(|w| {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(w);
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::utils::{remove_slices_from_string, trim_whitespace_v2};

    #[test]
    fn test_rm_slices() {
        assert_eq!(remove_slices_from_string("[aaaaaaaa]aa").unwrap(), "aa");
    }
    #[test]
    fn test_trim_whitespace() {
        assert_eq!(trim_whitespace_v2("   Hello     World!"), "Hello World!");
    }
}
