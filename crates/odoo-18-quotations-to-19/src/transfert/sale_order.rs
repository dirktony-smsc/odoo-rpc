use std::{collections::HashMap, num::NonZero};

use odoo_api_commons::{Command, Domain, PaginationParam, domain::operators::EQUALS_TO};
use odoo_json2::base_methods::{create::CreateParam, name_search::NameSearchParam};
use odoo_rpc::ModelName;

use crate::{
    client::Clients,
    error,
    models::{
        sales_order::{self, SalesOrderFrom18, SalesOrderTo19},
        sales_order_line::{
            SALES_ORDER_LINE_MODEL_NAME, SalesOrderLineFrom18, SalesOrderLineToOdoo19,
        },
    },
    utils::{
        partner_cache::PartnerMappingCache, remove_slices_from_string, tax_cache::TaxMappingCache,
        trim_whitespace_v2,
    },
};

pub async fn run_transfert(clients: &Clients, limit: NonZero<u32>) -> Result<(), error::Error> {
    let count = clients
        .odoo_18
        .search_count(SalesOrderFrom18::NAME.into(), Default::default())
        .await?;
    log::info!("Odoo 18 `sale.order` count = {count}");
    let mut order_lines_mappings = HashMap::<u64, u64>::new();
    let mut current_offset = 0u32;
    let mut tax_cache = TaxMappingCache::default();
    let mut partners_cache = PartnerMappingCache::default();
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
            log::info!("Order `{}` ({})", order.id, order.name);
            let new_order_id = {
                let new_order = {
                    log::debug!("Getting partner id of {:?}", order.partner_id);
                    let partner_id = partners_cache
                        .get_mapping(
                            clients,
                            order.partner_id.id,
                            Some(order.partner_id.name.clone()),
                        )
                        .await?;
                    log::debug!("Getting partner id of {:?}", order.partner_invoice_id);
                    let partner_invoice_id = partners_cache
                        .get_mapping(
                            clients,
                            order.partner_invoice_id.id,
                            Some(order.partner_invoice_id.name.clone()),
                        )
                        .await?;
                    log::debug!("Getting partner id of {:?}", order.partner_shipping_id);
                    let partner_shipping_id = partners_cache
                        .get_mapping(
                            clients,
                            order.partner_shipping_id.id,
                            Some(order.partner_shipping_id.name.clone()),
                        )
                        .await?;
                    SalesOrderTo19 {
                        name: order.name.clone(),
                        state: order.state,
                        partner_id,
                        client_order_ref: order.client_order_ref,
                        create_date: order.create_date,
                        commitment_date: order.commitment_date,
                        date_order: order.date_order,
                        origin: order.origin,
                        reference: order.reference,
                        partner_invoice_id,
                        partner_shipping_id,
                    }
                };
                let res = clients
                    .odoo_19
                    .create(
                        sales_order::SALES_ORDER_MODEL_NAME.into(),
                        CreateParam {
                            vals_list: vec![new_order],
                        },
                    )
                    .await?;
                log::debug!("new ids {:?}", res);
                *res.first().ok_or(error::Error::NothingCreated)?
            };
            log::info!("New order id {}", new_order_id);
            let search_domain = vec![Domain::condition("order_id", EQUALS_TO, order.id)];
            let count = clients
                .odoo_18
                .search_count(SalesOrderLineFrom18::NAME.into(), search_domain.clone())
                .await?;
            log::info!("Order `{}` lines: {}", order.id, count);
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
                        let new_order_line_id: u64 = {
                            let new_order_line = SalesOrderLineToOdoo19 {
                                name: order_line.name.clone(),
                                order_id: new_order_id,
                                sequence: order_line.sequence,
                                display_type: order_line.display_type,
                                is_downpayment: order_line.is_downpayment,
                                is_expense: order_line.is_expense,
                                product_id: if let Some(product) = order_line.product_id.as_ref() {
                                    log::debug!("Getting product id of {:?}", product);
                                    Some(
                                        clients
                                            .odoo_19
                                            .name_search(
                                                "product.product".into(),
                                                NameSearchParam {
                                                    name: trim_whitespace_v2(
                                                        &remove_slices_from_string(&product.name)?,
                                                    )
                                                    .into(),
                                                    limit: Some(1),
                                                    ..Default::default()
                                                },
                                            )
                                            .await?
                                            .first()
                                            .ok_or(error::Error::NotFound)?
                                            .0,
                                    )
                                } else {
                                    None
                                },
                                product_template_id: if let Some(product_template) =
                                    order_line.product_template_id.as_ref()
                                {
                                    log::debug!(
                                        "Getting product template id of {:?}",
                                        product_template
                                    );
                                    clients
                                        .odoo_19
                                        .name_search(
                                            "product.template".into(),
                                            NameSearchParam {
                                                name: product_template.name.clone().into(),
                                                limit: Some(1),
                                                ..Default::default()
                                            },
                                        )
                                        .await?
                                        .first()
                                        .map(|(id, _)| *id)
                                } else {
                                    None
                                },
                                linked_line_id: order_line.linked_line_id.as_ref().and_then(
                                    |order_line| order_lines_mappings.get(&order_line.id).copied(),
                                ),
                                tax_ids: if let Some(tax_ids) = order_line.tax_id.as_ref() {
                                    if tax_ids.is_empty() {
                                        Some(Vec::<Command<()>>::new())
                                    } else {
                                        let mut tax_ids_to_send = Vec::<u64>::new();
                                        for tax in tax_ids {
                                            log::debug!("Getting tax id of {:?}", tax);
                                            tax_ids_to_send
                                                .push(tax_cache.get_mapping(clients, *tax).await?);
                                        }
                                        Some(
                                            tax_ids_to_send
                                                .into_iter()
                                                .map(|id| Command::Link { id })
                                                .collect(),
                                        )
                                    }
                                } else {
                                    None
                                },
                                price_unit: order_line.price_unit,
                                customer_lead: order_line.customer_lead,
                                product_uom_qty: order_line.product_uom_qty,
                            };
                            let res = clients
                                .odoo_19
                                .create(
                                    SALES_ORDER_LINE_MODEL_NAME.into(),
                                    CreateParam {
                                        vals_list: vec![new_order_line],
                                    },
                                )
                                .await?;
                            log::debug!("new ids {:?}", res);
                            res.first().copied().ok_or(error::Error::NothingCreated)?
                        };
                        log::info!("new order line id: {new_order_line_id}");
                        order_lines_mappings.insert(order_line.id, new_order_line_id);
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
