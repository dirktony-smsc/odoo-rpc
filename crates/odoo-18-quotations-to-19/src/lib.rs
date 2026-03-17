pub mod config;

use std::fs;

use clap::Parser;

use config::Config;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[non_exhaustive]
pub struct Cli {
    #[arg(short, long)]
    pub configuration_file: String,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config: Config = toml::from_str(&fs::read_to_string(&cli.configuration_file)?)?;
    println!("{:#?}", config);
    Ok(())
}
