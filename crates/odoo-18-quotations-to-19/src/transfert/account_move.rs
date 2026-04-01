use std::num::NonZero;

use log::{debug, info, trace};
use odoo_api_commons::{
    Domain, PaginationParam,
    domain::operators::{EQUALS_TO, NOT_EQUALS_TO},
};
use odoo_rpc::ModelName;

use crate::{
    client::Clients,
    error,
    models::{
        account_move::{AccountMoveFromOdoo18, account_move_from_odoo_18_fields},
        account_move_line::AccountMoveLineFromOdoo18,
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
