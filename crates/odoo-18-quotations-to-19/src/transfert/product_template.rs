use std::num::NonZero;

use odoo_json2::base_methods::create::CreateParam;
use odoo_rpc::ModelName;

use crate::{
    client::Clients,
    error,
    models::product_template::{
        PRODUCT_TEMPLATE_MODEL_NAME, ProductTemplateFromOdoo18, ProductTemplateToOdoo19,
    },
    utils::{iterate_chunks::IterateModelFromOdoo18, product_tag_cache::ProductTagCache},
};

pub async fn run(
    clients: &Clients,
    limit: NonZero<u32>,
    default_uom_id: u64,
) -> Result<(), error::Error> {
    let mut stream = IterateModelFromOdoo18::new(
        &clients.odoo_18,
        ProductTemplateFromOdoo18::NAME.into(),
        ProductTemplateFromOdoo18::field_names(),
        Vec::default(),
        limit,
        0,
    )
    .await?;
    let mut product_tag_cache = ProductTagCache::default();

    while let Some(m_chunck) = stream.next::<ProductTemplateFromOdoo18>().await {
        let chunck = m_chunck?;
        for product_template in chunck {
            let to_import = ProductTemplateToOdoo19 {
                name: product_template.name,
                description: product_template.description,
                description_purchase: product_template.description_purchase,
                description_sale: product_template.description_sale,
                type_: product_template.type_,
                combo_ids: Default::default(),
                service_tracking: None,
                categ_id: Default::default(),
                currency_id: Default::default(),
                cost_currency_id: Default::default(),
                list_price: product_template.list_price,
                standard_price: product_template.standard_price,
                volume: product_template.volume,
                weight: product_template.weight,
                sale_ok: product_template.sale_ok,
                uom_id: Some(default_uom_id),
                color: product_template.color,
                attribute_line_ids: Default::default(),
                valid_product_template_attribute_line_ids: Default::default(),
                barcode: product_template.barcode,
                default_code: product_template.default_code,
                product_document_ids: Default::default(),
                product_tag_ids: {
                    let mut tag_commands =
                        Vec::<_>::with_capacity(product_template.product_tag_ids.len());
                    for tag_id in product_template.product_tag_ids {
                        tag_commands.push(odoo_api_commons::Command::Link {
                            id: product_tag_cache.get_mapping(clients, tag_id).await?,
                        });
                    }
                    tag_commands
                },
            };
            clients
                .odoo_19
                .create(
                    PRODUCT_TEMPLATE_MODEL_NAME.into(),
                    CreateParam {
                        vals_list: vec![to_import],
                    },
                )
                .await?;
        }
    }
    Ok(())
}
