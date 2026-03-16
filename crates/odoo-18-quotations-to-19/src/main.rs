use odoo_api_commons::*;
use odoo_json2::base_methods::{
    get_external_id::GetExternalIdParam, get_field_translations::GetFieldTranslationsParam,
    get_metadata::GetMetadataParam,
};
// use odoo_json2::base_methods::{
//     action_archive::ActionArchiveParam, action_unarchive::ActionUnarchiveParam, copy::CopyParam,
//     create::CreateParam, export_data::ExportDataParam, field_get::FieldsGetParam, read::ReadParam,
//     search::SearchParam, search_read::SearchReadParam, write::WriteParam,
// };
// use odoo_rpc::{ModelName, OdooJsonRPCClient};
use serde::{Deserialize, Serialize};
// use serde_json::json;
use std::{env::var, fs, vec};
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

// #[derive(Debug, Serialize)]
// struct TodoTask {
//     color: u8,
//     name: String,
// }

// impl ModelName for TodoTask {
//     const NAME: &'static str = "project.task";
// }

#[derive(Debug, Clone, Deserialize)]
struct TestPartner {
    email: String,
    active: bool,
}

#[derive(Debug, Deserialize, FieldNamesAsSlice)]
struct FieldGetOut {
    #[serde(rename = "type")]
    #[field_names_as_slice(skip)]
    _type: String,
    string: String,
    required: bool,
    depends: Vec<String>,
    exportable: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    env_logger::init();

    // log::info!("getting client...");
    // let client_18 = OdooJsonRPCClient::new(
    //     Url::parse(var("ODOO_18_BASE_URL")?.as_str())?,
    //     var("ODOO_18_USER")?,
    //     var("ODOO_18_PASSWORD")?,
    //     var("ODOO_18_DATABASE")?,
    // )
    // .await?;
    // log::info!("client got!");

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

    // let res = client_18
    //     .create_with_auto_module_name(vec![
    //         TodoTask {
    //             color: 8,
    //             name: "Hello from tony odoo-rpc-rs again".into(),
    //         },
    //         TodoTask {
    //             color: 2,
    //             name: "Just a seccond thing ....".into(),
    //         },
    //     ])
    //     .await?;
    // println!("{:#?}", res);

    // client_18
    //     .update(
    //         "project.task".into(),
    //         648,
    //         serde_json::json!({
    //             "description": fs::read_to_string("./lorem.html")?
    //         })
    //     )
    //     .await?;

    // client_18
    //     .unlink("project.task".into(), vec![647, 648])
    //     .await?;

    {
        let client_19 = odoo_json2::OdooJson2Client::builder()
            .api_key(var("ODOO_19_TEST_API_KEY")?)
            .base_url(Url::parse("http://localhost:8069")?)
            .build()?;

        // println!("{:#?}", client_19.version().await?);
        // println!(
        //     "{:#?}",
        //     client_19
        //         .export_data(
        //             "res.partner".into(),
        //             ExportDataParam {
        //                 ids: client_19
        //                     .search(
        //                         "res.partner".into(),
        //                         SearchParam {
        //                             pagination: PaginationParam {
        //                                 limit: 10.into(),
        //                                 ..Default::default()
        //                             }
        //                             .into(),
        //                             ..Default::default()
        //                         },
        //                     )
        //                     .await?,
        //                 fields_to_export: vec![
        //                     "name".into(),
        //                     "create_date".into(),
        //                     "phone".into(),
        //                     "email".into()
        //                 ],
        //                 context: None,
        //             },
        //         )
        //         .await?
        // );

        // println!(
        //     "{:#?}",
        //     client_19
        //         .fields_get::<FieldGetOut>(
        //             "res.partner".into(),
        //             FieldsGetParam {
        //                 attributes: Some({
        //                     let mut fields = FieldGetOut::FIELD_NAMES_AS_SLICE
        //                         .iter()
        //                         .map(|s| (**s).into())
        //                         .collect::<Vec<String>>();
        //                     fields.push("type".into());
        //                     fields
        //                 }),
        //                 ..Default::default()
        //             }
        //         )
        //         .await?
        // )
        println!(
            "{:#?}",
            client_19
                .get_metadata(
                    "res.partner".into(),
                    GetMetadataParam {
                        ids: vec![1710, 1709],
                        ..Default::default()
                    }
                )
                .await?
        )
    }

    Ok(())
}
