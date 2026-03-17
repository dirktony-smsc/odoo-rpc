use std::num::NonZero;

use odoo_api_commons::PaginationParam;
use odoo_rpc::ModelName;

use crate::{client::Clients, error, models::sales_order::SalesOrderFrom18};

pub async fn run_transfert(clients: &Clients, limit: NonZero<u32>) -> Result<(), error::Error> {
    let count = clients
        .odoo_18
        .search_count(SalesOrderFrom18::NAME.into(), Default::default())
        .await?;
    log::info!("Odoo 18 `sale.order` count = {count}");
    let mut current_offset = 0u32;
    loop {
        let sales = clients
            .odoo_18
            .search_read_with_auto_model_name_and_field_names::<SalesOrderFrom18>(
                Default::default(),
                PaginationParam {
                    offset: current_offset.into(),
                    limit: Some(limit.into()),
                },
            )
            .await?;

        log::info!("{:#?}", sales);
        {
            let next_offset: u32 = current_offset + Into::<u32>::into(limit);
            if next_offset as u64 > count {
                break;
            } else {
                current_offset = next_offset;
            }
        }
    }
    Ok(())
}
