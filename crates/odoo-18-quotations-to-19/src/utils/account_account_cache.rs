use std::collections::BTreeMap;

use odoo_api_commons::{Domain, PaginationParam, domain::operators::EQUALS_TO};
use odoo_json2::{
    OdooJson2Client,
    base_methods::{create::CreateParam, search::SearchParam},
};
use odoo_rpc::OdooJsonRPCClient;

use crate::{
    client::Clients,
    error,
    models::account_account::{
        ACCOUNT_ACCOUNT_MODEL_NAME, AccountAccountFromOdoo18, AccountAccountToOdoo19,
    },
};

pub async fn get_account_account_from_odoo18(
    client: &OdooJsonRPCClient,
    account_id: u64,
) -> Result<AccountAccountFromOdoo18, error::Error> {
    client
        .search_read_with_auto_model_name_and_field_names(
            vec![Domain::condition("id", EQUALS_TO, account_id)],
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

pub async fn create_account_account_to_odoo19(
    client: &OdooJson2Client,
    account: AccountAccountToOdoo19,
) -> Result<u64, error::Error> {
    client
        .create(
            ACCOUNT_ACCOUNT_MODEL_NAME.into(),
            CreateParam {
                vals_list: vec![account],
            },
        )
        .await?
        .into_iter()
        .next()
        .ok_or(error::Error::NothingCreated)
}

pub async fn get_or_create_account_account_from_18_to_19(
    clients: &Clients,
    account_id_from_18: u64,
) -> Result<u64, error::Error> {
    let account_from_odoo_18 =
        get_account_account_from_odoo18(&clients.odoo_18, account_id_from_18).await?;
    if let Some(id) = clients
        .odoo_19
        .search(
            ACCOUNT_ACCOUNT_MODEL_NAME.into(),
            SearchParam {
                domain: vec![Domain::condition(
                    "code",
                    EQUALS_TO,
                    account_from_odoo_18.code,
                )],
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
        create_account_account_to_odoo19(
            &clients.odoo_19,
            AccountAccountToOdoo19 {
                name: todo!(),
                currency_id: todo!(),
                code: todo!(),
                code_store: todo!(),
                placeholder_code: todo!(),
                deprecated: todo!(),
                used: todo!(),
                account_type: todo!(),
                include_initial_balance: todo!(),
                internal_group: todo!(),
                reconcile: todo!(),
                tax_ids: todo!(),
                note: todo!(),
                opening_debit: todo!(),
                opening_credit: todo!(),
                opening_balance: todo!(),
                current_balance: todo!(),
                related_taxes_amount: todo!(),
                non_trade: todo!(),
                display_mapping_tab: todo!(),
            },
        )
        .await
    }
}

#[derive(Debug, Default)]
pub struct AccountAccountMappingCache(BTreeMap<u64, u64>);

impl AccountAccountMappingCache {
    pub async fn get_mapping(
        &mut self,
        clients: &Clients,
        account_id_from_18: u64,
    ) -> Result<u64, error::Error> {
        match self.0.entry(account_id_from_18) {
            std::collections::btree_map::Entry::Occupied(occupied_entry) => {
                Ok(*occupied_entry.get())
            }
            std::collections::btree_map::Entry::Vacant(vacant_entry) => {
                let new_id =
                    get_or_create_account_account_from_18_to_19(clients, account_id_from_18)
                        .await?;
                Ok(*vacant_entry.insert(new_id))
            }
        }
    }
}
