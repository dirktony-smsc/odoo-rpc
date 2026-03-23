use std::num::NonZero;

use odoo_api_commons::PaginationParam;
use odoo_rpc::ModelName;
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::{
    client::Clients,
    error,
    models::crm_lead::{CrmLeadFromOdoo18, crm_lead_from_odoo_18_fields},
};

pub async fn run_transfert(clients: &Clients, limit: NonZero<u32>) -> Result<(), error::Error> {
    let count = clients
        .odoo_18
        .search_count(CrmLeadFromOdoo18::NAME.into(), Default::default())
        .await?;
    log::info!("Odoo 18 `{}` count = {count}", CrmLeadFromOdoo18::NAME);

    let mut current_offset = 0u32;

    loop {
        let leads = clients
            .odoo_18
            .search_read_with_auto_model_name::<CrmLeadFromOdoo18>(
                crm_lead_from_odoo_18_fields(),
                Default::default(),
                PaginationParam {
                    offset: current_offset.into(),
                    limit: Some(limit.into()),
                },
            )
            .await?;
        log::info!("{:#?}", leads);
        {
            let next_offset: u32 = current_offset + Into::<u32>::into(limit);
            if next_offset as u64 > count {
                break;
            } else {
                current_offset = next_offset;
            }
        }
    }
    Ok(())
}
