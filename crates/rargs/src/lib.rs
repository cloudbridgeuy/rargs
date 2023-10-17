use clap::{Parser, Subcommand};

pub mod commands;

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

#[derive(Debug, Parser)]
pub struct RunOptions {
    /// The path to the script
    pub script_root: String,
    /// An optional list of arguments to pass to the command
    #[arg(num_args(0..))]
    pub arguments: Option<Vec<String>>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Runs a rargs script
    Run(RunOptions),
    /// Build the script
    Build(BuildOptions),
}

impl From<RunOptions> for commands::run::Options {
    fn from(options: RunOptions) -> Self {
        Self {
            script_root: options.script_root,
            arguments: options.arguments,
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
