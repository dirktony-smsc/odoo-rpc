use std::fs;

use log::info;
use odoo_api_commons::PaginationParam;
use odoo_rpc::{ModelName, OdooJsonRPCClient};

use crate::{client::Clients, models::product_product::ProductProductFromOdoo18};

#[allow(unused)]
async fn get_o18_field_get<T: ModelName>(client: &OdooJsonRPCClient) -> anyhow::Result<()> {
    let a = client
        .fields_get(T::NAME.into(), Default::default(), Default::default())
        .await?;
    fs::write(
        format!("./target/{}.toml", T::NAME),
        toml::to_string_pretty(&a)?,
    )?;
    Ok(())
}

pub async fn run_some_dbg(clients: &Clients) -> anyhow::Result<()> {
    let a = clients
        .odoo_18
        .search_read_with_auto_model_name_and_field_names::<ProductProductFromOdoo18>(
            Default::default(),
            PaginationParam {
                limit: Some(30),
                ..Default::default()
            },
        )
        .await?;
    for product in a {
        info!("{:#?}", product);
    }
    Ok(())
}
