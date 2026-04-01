use std::num::NonZero;

use log::{debug, trace};
use odoo_api_commons::{Domain, PaginationParam, domain::operators::NOT_EQUALS_TO};
use odoo_rpc::ModelName;

use crate::{
    client::Clients,
    error,
    models::account_move::{AccountMoveFromOdoo18, account_move_from_odoo_18_fields},
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

    debug!("{count} account move found...");
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
        }
        {
            let next_offset = current_offset + limit;
            if (next_offset as u64) < count {
                current_offset = next_offset;
            } else {
                break;
            }
        }
    }

    Ok(())
}
