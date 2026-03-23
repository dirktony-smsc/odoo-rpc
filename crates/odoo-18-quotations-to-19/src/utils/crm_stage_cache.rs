use std::collections::HashMap;

use crate::{client::Clients, error, models::IdNameRepr, utils::get_or_create_by_name};

use odoo_json2::OdooJson2Client;

pub async fn get_or_create_crm_stage_by_name(
    client: &OdooJson2Client,
    name: String,
) -> Result<u64, error::Error> {
    get_or_create_by_name(client, "crm.stage".into(), name).await
}

#[derive(Debug, Default)]
pub struct CrmStageMappingCache(HashMap<u64, u64>);

impl CrmStageMappingCache {
    pub async fn get_mapping(
        &mut self,
        clients: &Clients,
        stage_id_from_18: u64,
        stage_name_from_18: Option<String>,
    ) -> Result<u64, error::Error> {
        match self.0.entry(stage_id_from_18) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                Ok(*occupied_entry.get())
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let new_id = get_or_create_crm_stage_by_name(
                    &clients.odoo_19,
                    if let Some(name) = stage_name_from_18 {
                        name
                    } else {
                        let stage_entry_from_18 = clients
                            .odoo_18
                            .read::<IdNameRepr>(
                                "crm.stage".into(),
                                vec![stage_id_from_18],
                                vec!["name".into()],
                            )
                            .await?
                            .into_iter()
                            .next()
                            .ok_or(error::Error::NotFound)?;
                        stage_entry_from_18.name
                    },
                )
                .await?;
                Ok(*vacant_entry.insert(new_id))
            }
        }
    }
}
