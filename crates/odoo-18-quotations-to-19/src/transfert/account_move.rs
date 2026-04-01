use std::num::NonZero;

use log::{debug, info, trace};
use odoo_api_commons::{
    Domain, PaginationParam,
    domain::operators::{EQUALS_TO, NOT_EQUALS_TO},
};
use odoo_json2::base_methods::create::CreateParam;
use odoo_rpc::ModelName;

use crate::{
    client::Clients,
    error,
    models::{
        account_move::{
            ACCOUNT_MOVE_MODEL_NAME, AccountMoveFromOdoo18, AccountMoveToOdoo19,
            account_move_from_odoo_18_fields,
        },
        account_move_line::AccountMoveLineFromOdoo18,
    },
    utils::{
        account_journal_cache::AccountJournalMappingCache, get_or_create_currency_by_name,
        partner_cache::PartnerMappingCache,
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
                                state: _move.state,
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
                for line in lines {
                    debug!("Account move line {:?}", line);
                }
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
