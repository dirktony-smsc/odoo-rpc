use std::{fs, path::PathBuf};

use clap::Parser;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[non_exhaustive]
pub struct Cli {
    #[arg(short, long)]
    pub configuration_file: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Odoo18Config {
    pub url: Url,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Odoo19Config {
    pub url: Url,
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct Config {
    pub odoo_18: Odoo18Config,
    pub odoo_19: Odoo19Config,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config: Config = toml::from_str(&fs::read_to_string(&cli.configuration_file)?)?;
    Ok(())
}
