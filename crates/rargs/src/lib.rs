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
#[clap(trailing_var_arg = true)]
pub struct RunOptions {
    /// The path to the script
    pub script_root: String,
    /// An optional list of arguments to pass to the command
    #[arg(allow_hyphen_values = true)]
    pub arguments: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct NewOptions {
    /// The name of the script
    pub name: String,
    /// An optional vesion number as a string
    #[arg(short, long)]
    pub version: Option<String>,
    /// An optional description of the script
    #[arg(short, long)]
    pub description: Option<String>,
    /// An optional author of the script
    #[arg(short, long)]
    pub author: Option<String>,
    /// Script destination directory
    #[arg(long)]
    pub destination: String,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Creates a new rargs minimal script
    New(NewOptions),
    /// Runs a rargs script
    Run(RunOptions),
    /// Build the script
    Build(BuildOptions),
}

impl From<NewOptions> for commands::new::Options {
    fn from(options: NewOptions) -> Self {
        Self {
            destination: options.destination,
            name: options.name,
            version: options.version,
            description: options.description,
            author: options.author,
        }
    }
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
