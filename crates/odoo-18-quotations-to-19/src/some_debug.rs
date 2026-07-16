use std::fs;

use odoo_rpc::ModelName;

use crate::{client::Clients, models::product_template::ProductTemplateFromOdoo18};

pub async fn run_some_dbg(clients: &Clients) -> anyhow::Result<()> {
    /*
    let a = clients
        .odoo_18
        .search_read_with_auto_model_name_and_field_names::<HrEmployeeFromOdoo18>(
            Default::default(),
            PaginationParam {
                limit: Some(10),
                ..Default::default()
            },
        )
        .await?;*/
    let a = clients
        .odoo_18
        .fields_get(
            ProductTemplateFromOdoo18::NAME.into(),
            Default::default(),
            Default::default(),
        )
        .await?;
    fs::write("./target/hr.employee.toml", toml::to_string_pretty(&a)?)?;
    Ok(())
}
