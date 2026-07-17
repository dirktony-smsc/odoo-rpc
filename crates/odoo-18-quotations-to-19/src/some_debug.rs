use std::fs;

use odoo_rpc::{ModelName, OdooJsonRPCClient};

use crate::{client::Clients, models::product_template::ProductTemplateFromOdoo18};

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
    // TODO refactor this to a generic function
    get_o18_field_get::<ProductTemplateFromOdoo18>(&clients.odoo_18).await?;
    Ok(())
}
