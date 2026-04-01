use odoo_json2::{OdooJson2Client, base_methods::name_search::NameSearchParam};

use crate::{
    error,
    utils::{remove_slices_from_string, trim_whitespace_v2},
};

pub async fn get_product_by_name(
    client: &OdooJson2Client,
    name: &str,
) -> Result<u64, error::Error> {
    Ok(client
        .name_search(
            "product.product".into(),
            NameSearchParam {
                name: trim_whitespace_v2(&remove_slices_from_string(name)?).into(),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await?
        .first()
        .ok_or(error::Error::NotFound)?
        .0)
}

pub async fn get_opt_product_template_by_name(
    client: &OdooJson2Client,
    name: &str,
) -> Result<Option<u64>, error::Error> {
    Ok(client
        .name_search(
            "product.template".into(),
            NameSearchParam {
                name: Some(name.into()),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await?
        .first()
        .map(|(id, _)| *id))
}
