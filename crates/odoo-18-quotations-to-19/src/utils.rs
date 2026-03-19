use odoo_json2::{
    OdooJson2Client,
    base_methods::{name_create::NameCreateParam, name_search::NameSearchParam},
};
use regex::Regex;
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::error;

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

pub fn remove_slices_from_string(i: String) -> Result<String, error::Error> {
    let s: String = Regex::new(r"\[[^)]*\]")?.replace(&i, "").into();
    log::debug!("{s}");
    Ok(s)
}
