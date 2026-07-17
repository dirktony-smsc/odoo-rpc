use std::collections::HashMap;

use odoo_api_commons::{Domain, PaginationParam};
use odoo_json2::base_methods::{
    create::CreateParam, search::SearchParam, search_read::SearchReadParam,
};
use odoo_rpc::ModelName;

use crate::{
    client::Clients,
    error,
    models::product_tag::{ProductTagFromOdoo18, ProductTagToOdoo19},
};

pub struct ProductTagCache(HashMap<u64, u64>);

pub async fn get_mapping_product_tag_o18_to_o19(
    clients: &Clients,
    odoo_18_id: u64,
) -> Result<u64, error::Error> {
    let partner_entry_from_18 = clients
        .odoo_18
        .read_with_auto_model_name_and_field_names::<ProductTagFromOdoo18>(vec![odoo_18_id])
        .await?
        .into_iter()
        .next()
        .ok_or(error::Error::NotFound)?;

    let new_id = {
        if let Some(o19_id) = clients
            .odoo_19
            .search(
                ProductTagFromOdoo18::NAME.into(),
                SearchParam {
                    domain: vec![Domain::condition(
                        "name",
                        "=",
                        partner_entry_from_18.name.as_str(),
                    )],
                    pagination: Some(PaginationParam {
                        offset: Some(1),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?
            .into_iter()
            .next()
        {
            o19_id
        } else {
            clients
                .odoo_19
                .create(
                    ProductTagFromOdoo18::NAME.into(),
                    CreateParam {
                        vals_list: vec![ProductTagToOdoo19 {
                            name: partner_entry_from_18.name,
                            sequence: partner_entry_from_18.sequence,
                            color: partner_entry_from_18.color,
                            visible_to_customers: false,
                        }],
                    },
                )
                .await?
                .into_iter()
                .next()
                .ok_or(error::Error::NothingCreated)?
        }
    };
    Ok(new_id)
}

impl ProductTagCache {
    pub async fn get_mapping(
        &mut self,
        clients: &Clients,
        partner_id_from_18: u64,
    ) -> Result<u64, error::Error> {
        match self.0.entry(partner_id_from_18) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                Ok(*occupied_entry.get())
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let new_id =
                    get_mapping_product_tag_o18_to_o19(clients, partner_id_from_18).await?;
                Ok(*vacant_entry.insert(new_id))
            }
        }
    }
}
