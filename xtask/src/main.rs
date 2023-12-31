//! See <https://github.com/matklad/cargo-xtask/>
//!
//! This binary defines various auxiliary build commands, which are not
//! expressible with just `cargo`.
//!
//! The binary is integrated into the `cargo` command line by using an
//! alias in `.cargo/config`.

mod cli;
mod scripts;

use clap::Parser;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    let cli = cli::App::parse();

    match &cli.command {
        Some(command) => match command {
            cli::Commands::DeployDocs(args) => scripts::deploy_docs(args),
            cli::Commands::Install(args) => scripts::install(args),
            cli::Commands::BuildDocs(args) => scripts::build_docs(args),
            cli::Commands::Release(args) => scripts::release(args),
            cli::Commands::DevDocs => scripts::doc_dev(),
        },
        None => {
            println!("No command specified.");
            std::process::exit(1);
        }
    }
}
