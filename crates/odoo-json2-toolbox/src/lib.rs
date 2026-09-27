use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
#[non_exhaustive]
pub struct Cli {
    #[arg(short)]
    /// The toolbox configuration file
    configuration_file: PathBuf,
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    PropertiesToField,
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    Ok(())
}
