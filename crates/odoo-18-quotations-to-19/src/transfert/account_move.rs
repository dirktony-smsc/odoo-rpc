use std::num::NonZero;

use log::{debug, info, trace, warn};
use odoo_api_commons::{
    Domain, PaginationParam,
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
        account_journal_cache::AccountJournalMappingCache, get_or_create_currency_by_name,
        partner_cache::PartnerMappingCache, product::get_product_by_name,
    },
};

pub async fn run_transfert(clients: &Clients, limit: NonZero<u32>) -> Result<(), error::Error> {
    let account_move_domains = vec![
        Domain::And,
        Domain::condition("name", NOT_EQUALS_TO, "/"),
        Domain::condition("name", NOT_EQUALS_TO, false),
    ];
    let limit = limit.get();
    let count = clients
        .odoo_18
        .search_count(
            AccountMoveFromOdoo18::NAME.into(),
            account_move_domains.clone(),
        )
        .await?;

    info!("{count} account move found...");
    trace!("Using pagination!");

    let mut current_offset = 0u32;
    let mut partner_cache = PartnerMappingCache::default();
    let mut journal_cache = AccountJournalMappingCache::default();

    loop {
        let account_moves = clients
            .odoo_18
            .search_read_with_auto_model_name::<AccountMoveFromOdoo18>(
                account_move_from_odoo_18_fields(),
                account_move_domains.clone(),
                PaginationParam {
                    offset: Some(current_offset),
                    limit: Some(limit),
                },
            )
            .await?;
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
            let count = clients
                .odoo_18
                .search_count(
                    AccountMoveLineFromOdoo18::NAME.into(),
                    account_move_line_domain.clone(),
                )
                .await?;
            info!("{count} lines for {}", _move.name);
            let mut current_offset = 0u32;
            loop {
                let lines = clients
                    .odoo_18
                    .search_read_with_auto_model_name_and_field_names::<AccountMoveLineFromOdoo18>(
                        account_move_line_domain.clone(),
                        PaginationParam {
                            offset: Some(current_offset),
                            limit: Some(limit),
                        },
                    )
                    .await?;
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
                    });
                }
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
                {
                    let next_offset = current_offset + limit;
                    if (next_offset as u64) < count {
                        current_offset = next_offset;
                        trace!("Loading next batch of account move line...");
                    } else {
                        break;
                    }
                }
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
        {
            let next_offset = current_offset + limit;
            if (next_offset as u64) < count {
                current_offset = next_offset;
                trace!("Loading next batch of account moves...");
            } else {
                break;
            }
        }
    }

    Ok(())
}
