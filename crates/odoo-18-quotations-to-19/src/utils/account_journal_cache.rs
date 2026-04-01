use std::collections::HashMap;

use crate::{client::Clients, error, models::IdNameRepr, utils::get_or_create_by_name};

use odoo_json2::OdooJson2Client;

pub async fn get_or_create_account_journal_by_name(
    client: &OdooJson2Client,
    name: String,
) -> Result<u64, error::Error> {
    get_or_create_by_name(client, "account.journal".into(), name).await
}

#[derive(Debug, Default)]
pub struct CrmStageMappingCache(HashMap<u64, u64>);

impl CrmStageMappingCache {
    pub async fn get_mapping(
        &mut self,
        clients: &Clients,
        journal_id_from_18: u64,
        journal_name_from_18: Option<String>,
    ) -> Result<u64, error::Error> {
        match self.0.entry(journal_id_from_18) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                Ok(*occupied_entry.get())
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let new_id = get_or_create_account_journal_by_name(
                    &clients.odoo_19,
                    if let Some(name) = journal_name_from_18 {
                        name
                    } else {
                        let journal_entry_from_18 = clients
                            .odoo_18
                            .read::<IdNameRepr>(
                                "account.journal".into(),
                                vec![journal_id_from_18],
                                vec!["name".into()],
                            )
                            .await?
                            .into_iter()
                            .next()
                            .ok_or(error::Error::NotFound)?;
                        journal_entry_from_18.name
                    },
                )
                .await?;
                Ok(*vacant_entry.insert(new_id))
            }
        }
    }
}
