pub mod client;
pub mod config;
pub mod error;
pub mod models;

use std::fs;

use clap::Parser;

use config::Config;

use crate::client::Clients;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[non_exhaustive]
pub struct Cli {
    #[arg(short, long)]
    pub configuration_file: String,
}

pub async fn run_transfert(clients: &Clients) -> anyhow::Result<()> {
    Ok(())
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

    run_transfert(&clients).await?;

    Ok(())
}
