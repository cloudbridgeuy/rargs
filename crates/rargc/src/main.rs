use clap::Parser;
use color_eyre::eyre::{self, Result};

use rargc::commands;
use rargc::{Cli, SubCommands};

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Some(SubCommands::Tree { script_root }) => {
            commands::tree::Command::new(commands::tree::Options { script_root }).run()
        }
        Some(SubCommands::Build { script_root }) => {
            commands::build::Command::new(commands::build::Options { script_root }).run()
        }
        None => Err(eyre::format_err!("No subcommand provided")),
    }
}
