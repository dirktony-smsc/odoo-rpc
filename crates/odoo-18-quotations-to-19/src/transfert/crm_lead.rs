use std::num::NonZero;

use odoo_api_commons::PaginationParam;
use odoo_json2::base_methods::create::CreateParam;
use odoo_rpc::ModelName;

use crate::{
    client::Clients,
    error,
    models::crm_lead::{
        CRM_LEAD_MODEL_NAME, CrmLeadFromOdoo18, CrmLeadToOdoo19, crm_lead_from_odoo_18_fields,
    },
    utils::{crm_stage_cache::CrmStageMappingCache, partner_cache::PartnerMappingCache},
};

pub async fn run_transfert(clients: &Clients, limit: NonZero<u32>) -> Result<(), error::Error> {
    let count = clients
        .odoo_18
        .search_count(CrmLeadFromOdoo18::NAME.into(), Default::default())
        .await?;
    log::info!("Odoo 18 `{}` count = {count}", CrmLeadFromOdoo18::NAME);

    let mut partner_cache = PartnerMappingCache::default();
    let mut crm_stage_cache = CrmStageMappingCache::default();

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
        {
            let mut to_import: Vec<CrmLeadToOdoo19> = Vec::with_capacity(leads.len());
            for lead in leads {
                to_import.push(CrmLeadToOdoo19 {
                    name: lead.name,
                    type_: lead.type_,
                    referred: lead.referred,
                    description: lead.description,
                    active: lead.active,
                    priority: lead.priority,
                    stage_id: if let Some(stage_id) = lead.stage_id {
                        Some(
                            crm_stage_cache
                                .get_mapping(clients, stage_id.id, Some(stage_id.name))
                                .await?,
                        )
                    } else {
                        None
                    },
                    color: lead.color,
                    expected_revenue: lead.expected_revenue,
                    prorated_revenue: lead.prorated_revenue,
                    recurring_revenue: lead.recurring_revenue,
                    recurring_revenue_monthly: lead.recurring_revenue_monthly,
                    recurring_revenue_monthly_prorated: lead.recurring_revenue_monthly_prorated,
                    recurring_revenue_prorated: lead.recurring_revenue_prorated,
                    date_closed: lead.date_closed,
                    date_automation_last: lead.date_automation_last,
                    date_open: lead.date_open,
                    date_conversion: lead.date_conversion,
                    date_deadline: lead.date_deadline,
                    partner_id: if let Some(partner_id) = lead.partner_id {
                        Some(
                            partner_cache
                                .get_mapping(clients, partner_id.id, Some(partner_id.name))
                                .await?,
                        )
                    } else {
                        None
                    },
                });
            }
            to_import.shrink_to_fit();
            let res = clients
                .odoo_19
                .create(
                    CRM_LEAD_MODEL_NAME.into(),
                    CreateParam {
                        vals_list: to_import,
                    },
                )
                .await?;
            log::info!("Imported {} leads/opportunities", res.len());
            log::debug!("New leads {:?}", res);
        }
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
