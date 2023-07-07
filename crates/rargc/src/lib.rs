use clap::{Parser, Subcommand};

pub mod commands;

#[derive(Debug, Parser)]
#[command(name = "v0.1.0")]
#[command(about = "A bash framework for managing your bash scripts")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Output a tree of all the commands available based on the script root
    Tree {
        /// The path to the script root
        script_root: String,
    },
    /// Build the script
    Build {
        /// The path to the script root
        script_root: String,
    },
}
