use std::num::NonZero;

use odoo_api_commons::{Domain, PaginationParam, domain::operators::EQUALS_TO};
use odoo_rpc::ModelName;

use crate::{
    client::Clients,
    error,
    models::{sales_order::SalesOrderFrom18, sales_order_line::SalesOrderLineFrom18},
};

pub async fn run_transfert(clients: &Clients, limit: NonZero<u32>) -> Result<(), error::Error> {
    let count = clients
        .odoo_18
        .search_count(SalesOrderFrom18::NAME.into(), Default::default())
        .await?;
    log::info!("Odoo 18 `sale.order` count = {count}");
    let mut current_offset = 0u32;
    loop {
        let orders = clients
            .odoo_18
            .search_read_with_auto_model_name_and_field_names::<SalesOrderFrom18>(
                Default::default(),
                PaginationParam {
                    offset: current_offset.into(),
                    limit: Some(limit.into()),
                },
            )
            .await?;
        for order in orders {
            log::info!("Order `{}` ({})", order.name, order.id);
            let search_domain = vec![Domain::condition("order_id", EQUALS_TO, order.id)];
            let count = clients
                .odoo_18
                .search_count(SalesOrderLineFrom18::NAME.into(), search_domain.clone())
                .await?;
            log::info!("Order `{}` lines: {}", order.name, count);
            {
                let mut current_offset = 0u32;
                loop {
                    let order_lines = clients
                        .odoo_18
                        .search_read_with_auto_model_name_and_field_names::<SalesOrderLineFrom18>(
                            search_domain.clone(),
                            PaginationParam {
                                offset: current_offset.into(),
                                limit: Some(limit.into()),
                            },
                        )
                        .await?;
                    for order_line in order_lines {
                        log::info!(
                            "Order `{}` line `{}` ({})",
                            order.name,
                            order_line.name,
                            order_line.id
                        );
                    }

                    {
                        {
                            let next_offset: u32 = current_offset + Into::<u32>::into(limit);
                            if next_offset as u64 > count {
                                break;
                            } else {
                                current_offset = next_offset;
                            }
                        }
                    }
                }
            }
        }
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
