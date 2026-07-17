use std::{fs, num::NonZero};

use log::info;
// use odoo_api_commons::PaginationParam;
use odoo_rpc::{ModelName, OdooJsonRPCClient};

use crate::{
    client::Clients,
    models::{
        // product_product::ProductProductFromOdoo18,
        product_template::ProductTemplateFromOdoo18,
    },
    utils::iterate_chunks::IterateModelFromOdoo18,
};

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
    let mut stream = IterateModelFromOdoo18::new(
        &clients.odoo_18,
        ProductTemplateFromOdoo18::NAME.into(),
        ProductTemplateFromOdoo18::field_names(),
        Default::default(),
        NonZero::new(30).unwrap(),
        0,
    )
    .await?;
    while let Some(maybe_chunck) = stream.next::<ProductTemplateFromOdoo18>().await {
        for product in maybe_chunck? {
            info!("{:#?}", product);
        }
    }
    Ok(())
}
