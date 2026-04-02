use std::collections::BTreeMap;

use crate::{
    client::Clients,
    error,
    models::account_journal::{
        ACCOUNT_JOURNAL_MODEL_NAME, AccountJournalFromOdoo18, AccountJournalToOdoo19,
        account_journal_from_odoo_18_fields,
    },
};

use odoo_api_commons::{Domain, PaginationParam, domain::operators::EQUALS_TO};
use odoo_json2::{
    OdooJson2Client,
    base_methods::{create::CreateParam, search::SearchParam},
};
use odoo_rpc::OdooJsonRPCClient;

pub async fn get_account_journal_from_odoo18(
    client: &OdooJsonRPCClient,
    journal_id: u64,
) -> Result<AccountJournalFromOdoo18, error::Error> {
    client
        .search_read_with_auto_model_name(
            account_journal_from_odoo_18_fields(),
            vec![Domain::condition("id", EQUALS_TO, journal_id)],
            PaginationParam {
                limit: Some(1),
                ..Default::default()
            },
        )
        .await?
        .into_iter()
        .next()
        .ok_or(error::Error::NotFound)
}

pub async fn create_account_journal_to_odoo19(
    client: &OdooJson2Client,
    journal: AccountJournalToOdoo19,
) -> Result<u64, error::Error> {
    client
        .create(
            ACCOUNT_JOURNAL_MODEL_NAME.into(),
            CreateParam {
                vals_list: vec![journal],
            },
        )
        .await?
        .into_iter()
        .next()
        .ok_or(error::Error::NothingCreated)
}

pub async fn get_or_create_account_journal_from_18_to_19(
    clients: &Clients,
    journal_id_from_18: u64,
) -> Result<u64, error::Error> {
    let journal_from_odoo_18 =
        get_account_journal_from_odoo18(&clients.odoo_18, journal_id_from_18).await?;
    if let Some(id) = clients
        .odoo_19
        .search(
            ACCOUNT_JOURNAL_MODEL_NAME.into(),
            SearchParam {
                domain: vec![
                    Domain::And,
                    Domain::condition("name", EQUALS_TO, journal_from_odoo_18.name.as_str()),
                    Domain::condition("code", EQUALS_TO, journal_from_odoo_18.code.as_str()),
                ],
                pagination: Some(PaginationParam {
                    limit: Some(1),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?
        .into_iter()
        .next()
    {
        Ok(id)
    } else {
        create_account_journal_to_odoo19(
            &clients.odoo_19,
            AccountJournalToOdoo19 {
                name: journal_from_odoo_18.name,
                code: journal_from_odoo_18.code,
                active: journal_from_odoo_18.active,
                type_: journal_from_odoo_18.type_,
                // autocheck_on_post: journal_from_odoo_18.autocheck_on_post,
                restrict_mode_hash_table: journal_from_odoo_18.restrict_mode_hash_table,
                sequence: journal_from_odoo_18.sequence,
                invoice_reference_type: journal_from_odoo_18.invoice_reference_type,
                invoice_reference_model: journal_from_odoo_18.invoice_reference_model,
            },
        )
        .await
    }
}

#[derive(Debug, Default)]
pub struct AccountJournalMappingCache(BTreeMap<u64, u64>);

impl AccountJournalMappingCache {
    pub async fn get_mapping(
        &mut self,
        clients: &Clients,
        journal_id_from_18: u64,
    ) -> Result<u64, error::Error> {
        match self.0.entry(journal_id_from_18) {
            std::collections::btree_map::Entry::Occupied(occupied_entry) => {
                Ok(*occupied_entry.get())
            }
            std::collections::btree_map::Entry::Vacant(vacant_entry) => {
                let new_id =
                    get_or_create_account_journal_from_18_to_19(clients, journal_id_from_18)
                        .await?;
                Ok(*vacant_entry.insert(new_id))
            }
        }
    }
}
