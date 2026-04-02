use std::collections::BTreeMap;

use crate::{client::Clients, error, models::IdNameRepr, utils::get_or_create_partner_by_name};

#[derive(Debug, Default)]
pub struct PartnerMappingCache(BTreeMap<u64, u64>);

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
