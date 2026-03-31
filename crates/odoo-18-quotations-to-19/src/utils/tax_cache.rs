use std::collections::HashMap;

use odoo_api_commons::Domain;
use odoo_json2::base_methods::search::SearchParam;

use crate::{client::Clients, error, models::account_tax::AccountTax1};

#[derive(Debug, Default)]
pub struct TaxMappingCache(HashMap<u64, u64>);

impl TaxMappingCache {
    pub async fn get_mapping(
        &mut self,
        clients: &Clients,
        tax_from_18: u64,
    ) -> Result<u64, error::Error> {
        match self.0.entry(tax_from_18) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                Ok(*occupied_entry.get())
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let tax_entry_from_18 = clients
                    .odoo_18
                    .read_with_auto_model_name_and_field_names::<AccountTax1>(vec![tax_from_18])
                    .await?
                    .into_iter()
                    .next()
                    .ok_or(error::Error::NotFound)?;
                let tax_from_19 = clients
                    .odoo_19
                    .search(
                        "account.tax".into(),
                        SearchParam {
                            domain: vec![
                                Domain::condition("amount", "=", tax_entry_from_18.amount),
                                Domain::condition(
                                    "type_tax_use",
                                    "=",
                                    serde_json::to_value(tax_entry_from_18.type_tax_use)?,
                                ),
                            ],
                            ..Default::default()
                        },
                    )
                    .await?
                    .first()
                    .copied()
                    .ok_or(error::Error::NotFound)?;
                Ok(*vacant_entry.insert(tax_from_19))
            }
        }
    }
}
