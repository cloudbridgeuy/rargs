use anyhow::Result;
use clap::Parser;

use rargc::commands;
use rargc::{Cli, SubCommands};

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Some(SubCommands::Tree { script_root }) => {
            commands::tree::Command::new(commands::tree::Options { script_root }).run()
        }
        None => Err(anyhow::format_err!("No subcommand provided")),
    }
}

