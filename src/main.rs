use odoo_18_quotations_to_19::{
    odoo_18::{ModelName, Odoo18JsonRPCClient},
    utils::{Domain, PaginationParam, deserialize_and_default_if_false},
};
use serde::Deserialize;
use std::env::var;
use struct_field_names_as_array::FieldNamesAsSlice;
use url::Url;

#[derive(Debug, Deserialize, FieldNamesAsSlice)]
struct Partner {
    id: u64,
    name: String,
    #[serde(default, deserialize_with = "deserialize_and_default_if_false")]
    email: Option<String>,
}

impl ModelName for Partner {
    const NAME: &'static str = "res.partner";
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    env_logger::init();

    log::info!("getting client...");
    let client_18 = Odoo18JsonRPCClient::new(
        Url::parse(var("ODOO_18_BASE_URL")?.as_str())?,
        var("ODOO_18_USER")?,
        var("ODOO_18_PASSWORD")?,
        var("ODOO_18_DATABASE")?,
    )
    .await?;
    log::info!("client got!");

    println!("uid = {:#?}", client_18.get_uid());
    println!("version = {:#?}", client_18.version().await?);

    let a = client_18
        .search_read_typed_1::<Partner>(
            vec![Domain::new("is_company", "=", true)],
            PaginationParam {
                offset: 0.into(),
                limit: 10.into(),
            },
        )
        .await?;
    let count = client_18
        .search_count(
            "res.partner".into(),
            vec![Domain::new("is_company", "=", true)],
        )
        .await?;
    println!("count {count}");
    println!("{:#?}", a);

    Ok(())
}
