use std::collections::BTreeMap;

use crate::{client::Clients, error, models::IdNameRepr, utils::get_or_create_partner_by_name};

#[derive(Debug, Default)]
pub struct PartnerMappingCache(BTreeMap<u64, u64>);

pub async fn get_mapping_partner_o18_to_o19(
    clients: &Clients,
    odoo_18_id: u64,
) -> Result<u64, error::Error> {
    let partner_entry_from_18 = clients
        .odoo_18
        .read::<IdNameRepr>("res.partner".into(), vec![odoo_18_id], vec!["name".into()])
        .await?
        .into_iter()
        .next()
        .ok_or(error::Error::NotFound)?;

    let new_id =
        get_or_create_partner_by_name(&clients.odoo_19, partner_entry_from_18.name).await?;
    Ok(new_id)
}

impl PartnerMappingCache {
    pub async fn get_mapping(
        &mut self,
        clients: &Clients,
        partner_id_from_18: u64,
        partner_name_from_18: Option<String>,
    ) -> Result<u64, error::Error> {
        match self.0.entry(partner_id_from_18) {
            std::collections::btree_map::Entry::Occupied(occupied_entry) => {
                Ok(*occupied_entry.get())
            }
            std::collections::btree_map::Entry::Vacant(vacant_entry) => {
                let new_id = get_or_create_partner_by_name(
                    &clients.odoo_19,
                    if let Some(name) = partner_name_from_18 {
                        name
                    } else {
                        let partner_entry_from_18 = clients
                            .odoo_18
                            .read::<IdNameRepr>(
                                "res.partner".into(),
                                vec![partner_id_from_18],
                                vec!["name".into()],
                            )
                            .await?
                            .into_iter()
                            .next()
                            .ok_or(error::Error::NotFound)?;
                        partner_entry_from_18.name
                    },
                )
                .await?;
                Ok(*vacant_entry.insert(new_id))
            }
        }
    }
}
