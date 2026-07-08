pub mod batch_update;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod transfert;
pub(crate) mod utils;

use std::{fs, num::NonZero};

use clap::{Parser, Subcommand};

use config::Config;
use odoo_rpc::ModelName;

use crate::{batch_update::BatchUpdateArg, models::hr_employee::HrEmployeeFromOdoo18};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
#[non_exhaustive]
pub struct Cli {
    #[arg(short)]
    pub limit: Option<NonZero<u32>>,
    #[arg(short, long)]
    pub configuration_file: String,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
#[non_exhaustive]
pub enum Commands {
    SaleOrder,
    CrmLead,
    AccountMove,
    BatchUpdate(BatchUpdateArg),
    SomeDebug,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config: Config = toml::from_str(&fs::read_to_string(&cli.configuration_file)?)?;

    let clients = client::Clients::from_config(config).await?;

    {
        let info = clients.odoo_18.version().await?;
        log::info!("Odoo JSON RPC version: {}", info.server_version);
    }
    {
        let info = clients.odoo_19.version().await?;
        log::info!("Odoo JSON2 API version: {}", info.version);
    }

    let limit = cli.limit.unwrap_or(NonZero::new(30).unwrap());

    match cli.command {
        Commands::SaleOrder => {
            transfert::sale_order::run_transfert(&clients, limit).await?;
        }
        Commands::CrmLead => {
            transfert::crm_lead::run_transfert(&clients, limit).await?;
        }
        Commands::SomeDebug => {
            /*
            let a = clients
                .odoo_18
                .search_read_with_auto_model_name_and_field_names::<HrEmployeeFromOdoo18>(
                    Default::default(),
                    PaginationParam {
                        limit: Some(10),
                        ..Default::default()
                    },
                )
                .await?;*/
            let a = clients
                .odoo_18
                .fields_get(
                    HrEmployeeFromOdoo18::NAME.into(),
                    Default::default(),
                    Default::default(),
                )
                .await?;
            fs::write("./target/hr.employee.toml", toml::to_string_pretty(&a)?)?;
        }
        Commands::BatchUpdate(arg) => {
            arg.run(&clients).await?;
        }
        Commands::AccountMove => {
            transfert::account_move::run_transfert(&clients, limit).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use crate::Cli;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
