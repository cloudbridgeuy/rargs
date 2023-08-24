use clap::{Parser, Subcommand};

pub mod commands;
pub mod param;
pub mod parser;
pub mod script;
pub mod templates;
pub mod token;

#[derive(Debug, Parser)]
#[command(name = "v0.1.0")]
#[command(about = "A bash framework for managing your bash scripts")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Debug, Parser)]
pub struct TreeOptions {
    /// The path to the script root
    pub script_root: String,
}

#[derive(Debug, Parser)]
pub struct BuildOptions {
    /// The path to the script root
    pub script_root: String,
    /// Script destination directory
    #[arg(short, long)]
    pub destination: String,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Output a tree of all the commands available based on the script root
    Tree(TreeOptions),
    /// Build the script
    Build(BuildOptions),
}

impl From<TreeOptions> for commands::tree::Options {
    fn from(options: TreeOptions) -> Self {
        Self {
            script_root: options.script_root,
        }
    }
}

impl From<BuildOptions> for commands::build::Options {
    fn from(options: BuildOptions) -> Self {
        Self {
            script_root: options.script_root,
            destination: options.destination,
        }
    }
}
