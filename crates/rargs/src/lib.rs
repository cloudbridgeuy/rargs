use clap::{Parser, Subcommand};
use clap_stdin::FileOrStdin;

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
#[clap(disable_help_flag = true)]
pub struct RunOptions {
    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
    /// Path to the script or read from stdin.
    #[clap(default_value = "-")]
    pub script: FileOrStdin,
    /// An optional list of arguments to pass to the command
    #[arg(allow_hyphen_values = true)]
    pub arguments: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct NewOptions {
    /// The name of the script
    pub name: String,
    /// An optional version number as a string
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
            script: options
                .script
                .contents()
                .expect("Unable to read script or stdin"),
            arguments: options.arguments,
        }
    }
}

impl From<BuildOptions> for commands::build::Options {
    fn from(options: BuildOptions) -> Self {
        Self {
            script: options.script_root,
            destination: options.destination,
        }
    }
}
