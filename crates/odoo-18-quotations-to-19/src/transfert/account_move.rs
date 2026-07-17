use std::num::NonZero;

use log::{debug, info, trace, warn};
use odoo_api_commons::{
    Domain,
    domain::operators::{EQUALS_TO, NOT_EQUALS_TO},
};
use odoo_json2::base_methods::{create::CreateParam, write::WriteParam};
use odoo_rpc::ModelName;
use serde_json::json;

use crate::{
    client::Clients,
    error,
    models::{
        account_move::{
            ACCOUNT_MOVE_MODEL_NAME, AccountMoveFromOdoo18, AccountMoveToOdoo19,
            account_move_from_odoo_18_fields,
        },
        account_move_line::{
            ACCOUNT_MOVE_LINE_MODEL_NAME, AccountMoveLineFromOdoo18, AccountMoveLineToOdoo19,
        },
    },
    utils::{
        FieldnamesAsStringVec, account_account_cache::AccountAccountMappingCache,
        account_journal_cache::AccountJournalMappingCache, get_or_create_currency_by_name,
        iterate_chunks::IterateModelFromOdoo18, partner_cache::PartnerMappingCache,
        product::get_product_by_name,
    },
};

pub async fn run_transfert(clients: &Clients, limit: NonZero<u32>) -> Result<(), error::Error> {
    let account_move_domains = vec![
        Domain::And,
        Domain::condition("name", NOT_EQUALS_TO, "/"),
        Domain::condition("name", NOT_EQUALS_TO, false),
    ];
    // let limit = limit.get();
    // let count = clients
    //     .odoo_18
    //     .search_count(
    //         AccountMoveFromOdoo18::NAME.into(),
    //         account_move_domains.clone(),
    //     )
    //     .await?;
    let mut acc_moves_stream = IterateModelFromOdoo18::new(
        &clients.odoo_18,
        AccountMoveFromOdoo18::NAME.into(),
        account_move_from_odoo_18_fields(),
        account_move_domains,
        limit,
        0,
    )
    .await?;
    info!("{} account move found...", acc_moves_stream.count());
    trace!("Using pagination!");

    let mut partner_cache = PartnerMappingCache::default();
    let mut journal_cache = AccountJournalMappingCache::default();
    let mut account_cache = AccountAccountMappingCache::default();

    while let Some(account_moves) = acc_moves_stream.next::<AccountMoveFromOdoo18>().await {
        let account_moves = account_moves?;
        for _move in account_moves {
            debug!("Account move {:#?} (ID: {})", _move.name, _move.id);

            let new_move_id = {
                clients
                    .odoo_19
                    .create(
                        ACCOUNT_MOVE_MODEL_NAME.into(),
                        CreateParam {
                            vals_list: vec![AccountMoveToOdoo19 {
                                name: _move.name.clone(),
                                name_placeholder: _move.name_placeholder,
                                ref_: _move.ref_,
                                date: _move.date,
                                state: Default::default(),
                                move_type: _move.move_type,
                                is_storno: _move.is_storno,
                                journal_id: journal_cache
                                    .get_mapping(clients, _move.journal_id.id)
                                    .await?,
                                auto_post: _move.auto_post,
                                currency_id: get_or_create_currency_by_name(
                                    &clients.odoo_19,
                                    _move.currency_id.name,
                                )
                                .await?,
                                partner_id: if let Some(partner_id) = _move.partner_id {
                                    Some(
                                        partner_cache
                                            .get_mapping(
                                                clients,
                                                partner_id.id,
                                                Some(partner_id.name),
                                            )
                                            .await?,
                                    )
                                } else {
                                    None
                                },
                            }],
                        },
                    )
                    .await?
                    .into_iter()
                    .next()
                    .ok_or(error::Error::NothingCreated)?
            };

            let account_move_line_domain = vec![Domain::condition("move_id", EQUALS_TO, _move.id)];
            let mut lines_stream = IterateModelFromOdoo18::new(
                &clients.odoo_18,
                AccountMoveLineFromOdoo18::NAME.into(),
                AccountMoveLineFromOdoo18::field_names_as_string_vec(),
                account_move_line_domain,
                limit,
                0,
            )
            .await?;
            while let Some(maybe_lines) = lines_stream.next::<AccountMoveLineFromOdoo18>().await {
                let lines = maybe_lines?;
                let mut to_import_lines =
                    Vec::<AccountMoveLineToOdoo19>::with_capacity(lines.len());
                for line in lines {
                    to_import_lines.push(AccountMoveLineToOdoo19 {
                        move_id: new_move_id,
                        debit: line.debit,
                        credit: line.credit,
                        currency_id: if let Some(currency) = line.currency_id {
                            Some(
                                get_or_create_currency_by_name(&clients.odoo_19, currency.name)
                                    .await?,
                            )
                        } else {
                            None
                        },
                        partner_id: if let Some(partner) = line.partner_id {
                            Some(
                                partner_cache
                                    .get_mapping(clients, partner.id, Some(partner.name))
                                    .await?,
                            )
                        } else {
                            None
                        },
                        display_type: line.display_type,
                        product_id: if let Some(product) = line.product_id {
                            Some(get_product_by_name(&clients.odoo_19, &product.name).await?)
                        } else {
                            None
                        },
                        quantity: line.price_unit,
                        price_unit: line.price_unit,
                        discount: line.discount,
                        is_refund: line.is_refund,
                        account_id: if let Some(account) = line.account_id {
                            Some(account_cache.get_mapping(clients, account.id).await?)
                        } else {
                            None
                        },
                    });
                }
                trace!("to import {:#?}", to_import_lines);
                if to_import_lines.is_empty() {
                    warn!("No account lines needs to be imported. Moving on...");
                } else {
                    let new_ids = clients
                        .odoo_19
                        .create(
                            ACCOUNT_MOVE_LINE_MODEL_NAME.into(),
                            CreateParam {
                                vals_list: to_import_lines,
                            },
                        )
                        .await?;
                    if new_ids.is_empty() {
                        return Err(error::Error::NothingCreated);
                    } else {
                        log::info!("inserted {:?} line for {}", new_ids, new_move_id);
                    }
                };
            }
            {
                clients
                    .odoo_19
                    .write(
                        ACCOUNT_MOVE_MODEL_NAME.into(),
                        WriteParam {
                            ids: [new_move_id].into(),
                            vals: json!({
                                "state": _move.state
                            }),
                        },
                    )
                    .await?;
                log::debug!("updating state")
            }
        }
    }

    Ok(())
}
