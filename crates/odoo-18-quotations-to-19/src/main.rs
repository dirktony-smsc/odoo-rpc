use odoo_rpc::{
    ModelName, OdooJsonRPCClient,
    utils::{Domain, PaginationParam, deserialize_and_default_if_false},
};
use serde::{Deserialize, Serialize};
use std::env::var;
use struct_field_names_as_array::FieldNamesAsSlice;
use url::Url;

// #[derive(Debug, Deserialize, FieldNamesAsSlice)]
// struct Partner {
//     id: u64,
//     name: String,
//     #[serde(default, deserialize_with = "deserialize_and_default_if_false")]
//     email: Option<String>,
// }

// impl ModelName for Partner {
//     const NAME: &'static str = "res.partner";
// }

#[derive(Debug, Serialize)]
struct TodoTask {
    color: u8,
    name: String,
}

impl ModelName for TodoTask {
    const NAME: &'static str = "project.task";
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    env_logger::init();

    log::info!("getting client...");
    let client_18 = OdooJsonRPCClient::new(
        Url::parse(var("ODOO_18_BASE_URL")?.as_str())?,
        var("ODOO_18_USER")?,
        var("ODOO_18_PASSWORD")?,
        var("ODOO_18_DATABASE")?,
    )
    .await?;
    log::info!("client got!");

    // println!("uid = {:#?}", client_18.get_uid());
    // println!("version = {:#?}", client_18.version().await?);

    // {
    //     let a = client_18
    //         .search_read_with_auto_model_name_and_field_names::<Partner>(
    //             vec![Domain::new("is_company", "=", true)],
    //             PaginationParam {
    //                 offset: 0.into(),
    //                 limit: 10.into(),
    //             },
    //         )
    //         .await?;
    //     let count = client_18
    //         .search_count(
    //             "res.partner".into(),
    //             vec![Domain::new("is_company", "=", true)],
    //         )
    //         .await?;
    //     println!("count {count}");
    //     println!("{:#?}", a);
    // }
    // {
    //     let ids = client_18
    //         .search(
    //             Partner::NAME.into(),
    //             vec![Domain::new("is_company", "=", true)],
    //             PaginationParam {
    //                 offset: 10.into(),
    //                 limit: 20.into(),
    //             },
    //         )
    //         .await?;
    //     let a: Vec<Partner> = client_18
    //         .read_with_auto_model_name_and_field_names(ids)
    //         .await?;
    //     println!("{:#?}", a);
    // }
    // println!(
    //     "{:#?}",
    //     client_18
    //         .fields_get(Partner::NAME.into(), Default::default(), Default::default())
    //         .await?
    // );
    let res = client_18
        .create_with_auto_module_name(vec![
            TodoTask {
                color: 8,
                name: "Hello from tony odoo-rpc-rs again".into(),
            },
            TodoTask {
                color: 2,
                name: "Just a seccond thing ....".into(),
            },
        ])
        .await?;
    println!("{:#?}", res);
    Ok(())
}
