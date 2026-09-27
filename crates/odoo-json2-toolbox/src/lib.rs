mod client;
pub(crate) mod commands;
pub(crate) mod config;

use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

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
    PropertiesToField(commands::properties_to_field::PropertiesToFieldArg),
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config: config::ConfigFile = parse_cli_config(&cli)?;

    let client = client::create_client(config)?;

    match cli.command {
        CliCommand::PropertiesToField(properties_to_field_arg) => {
            commands::properties_to_field::properties_to_field(&client, properties_to_field_arg)
                .await?;
        }
    }

    Ok(())
}

fn parse_cli_config(cli: &Cli) -> Result<config::ConfigFile, anyhow::Error> {
    let file = File::open(&cli.configuration_file)?;
    let mut buf = {
        if let Some(len) = file.metadata().ok().map(|metadata| metadata.len()) {
            Vec::<u8>::with_capacity(len.try_into()?)
        } else {
            Vec::new()
        }
    };
    let mut s = BufReader::new(file);
    s.read_to_end(&mut buf)?;
    Ok(toml::from_slice(&buf)?)
}
