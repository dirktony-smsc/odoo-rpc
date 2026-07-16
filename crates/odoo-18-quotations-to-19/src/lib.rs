pub mod batch_update;
pub mod client;
pub mod config;
pub mod error;
pub mod models;
#[cfg(debug_assertions)]
pub mod some_debug;
pub mod transfert;
pub(crate) mod utils;

use std::{fs, num::NonZero};

use clap::{Parser, Subcommand};

use config::Config;

use crate::{
    batch_update::BatchUpdateArg,
    transfert::res_partner_18_fields_to_19_properties::ResPartner18FieldsTo19PropertiesArg,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
#[non_exhaustive]
pub struct Cli {
    #[arg(short)]
    pub limit: Option<NonZero<u32>>,
    /// Configuration file
    ///
    /// Defaults to `default.conf.toml` if not set.
    #[arg(short, long)]
    pub configuration_file: Option<String>,
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
    #[cfg(debug_assertions)]
    SomeDebug,
    ResPartner18FieldsTo19Properties(ResPartner18FieldsTo19PropertiesArg),
}

const DEFAULT_CONF_PATH: &str = "default.conf.toml";

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config: Config = toml::from_str(&fs::read_to_string(
        cli.configuration_file
            .as_deref()
            .unwrap_or(DEFAULT_CONF_PATH),
    )?)?;

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
        #[cfg(debug_assertions)]
        Commands::SomeDebug => {
            some_debug::run_some_dbg(&clients).await?;
        }
        Commands::BatchUpdate(arg) => {
            arg.run(&clients).await?;
        }
        Commands::AccountMove => {
            transfert::account_move::run_transfert(&clients, limit).await?;
        }
        Commands::ResPartner18FieldsTo19Properties(arg) => {
            arg.run(&clients).await?;
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
