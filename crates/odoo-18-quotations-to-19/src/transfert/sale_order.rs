use std::{collections::HashMap, num::NonZero};

use odoo_api_commons::{Command, Domain, domain::operators::EQUALS_TO};
use odoo_json2::base_methods::create::CreateParam;
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
        FieldnamesAsStringVec,
        iterate_chunks::IterateModelFromOdoo18,
        partner_cache::PartnerMappingCache,
        product::{get_opt_product_template_by_name, get_product_by_name},
        tax_cache::TaxMappingCache,
    },
};

pub async fn run_transfert(clients: &Clients, limit: NonZero<u32>) -> Result<(), error::Error> {
    let mut stream = IterateModelFromOdoo18::new(
        &clients.odoo_18,
        SalesOrderFrom18::NAME.into(),
        SalesOrderFrom18::field_names_as_string_vec(),
        Default::default(),
        limit,
        0,
    )
    .await?;
    log::info!("Odoo 18 `sale.order` count = {}", stream.count());
    let mut order_lines_mappings = HashMap::<u64, u64>::new();
    let mut tax_cache = TaxMappingCache::default();
    let mut partners_cache = PartnerMappingCache::default();
    while let Some(maybe_orders) = stream.next::<SalesOrderFrom18>().await {
        let orders = maybe_orders?;
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
            let mut stream = IterateModelFromOdoo18::new(
                &clients.odoo_18,
                SalesOrderLineFrom18::NAME.into(),
                SalesOrderLineFrom18::field_names_as_string_vec(),
                search_domain,
                limit,
                0,
            )
            .await?;
            log::info!("Order `{}` lines: {}", order.id, stream.count());
            {
                while let Some(maybe_order_lines) = stream.next::<SalesOrderLineFrom18>().await {
                    let order_lines = maybe_order_lines?;
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
                                        get_product_by_name(&clients.odoo_19, &product.name)
                                            .await?,
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
                                    get_opt_product_template_by_name(
                                        &clients.odoo_19,
                                        &product_template.name,
                                    )
                                    .await?
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
                }
            }
        }
    }
    Ok(())
}
