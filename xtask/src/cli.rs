use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "xtasks")]
#[command(about = "Run project tasks using rust instead of scripts")]
pub struct App {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Deploy the documentation to GCP
    DeployDocs(DeployDocsArgs),
    /// Build the documentation site
    BuildDocs(BuildDocsArgs),
}

#[derive(Args, Debug)]
pub struct BuildDocsArgs {
    /// Path to the directory where the documentation site will be built
    #[arg(short, long, default_value = "web")]
    pub path: String,
    /// Base URL to use for the docs
    #[arg(short, long, default_value = "rargs.cloudbridge.uy")]
    pub base_url: String,
    /// Output directory for the built site
    #[arg(short, long, default_value = "public")]
    pub output_dir: String,
}

#[derive(Args, Debug)]
pub struct DeployDocsArgs {
    /// URL where the documentation site will be published
    #[arg(short, long, default_value = "rargs.cloudbridge.uy")]
    pub url: String,
    /// Name of the gcloud configuration to use
    #[arg(short, long, default_value = "cloudbridgeuy")]
    pub gcp_config: String,
    /// Path to the directory where the documentation site will be built
    #[arg(short, long, default_value = "web")]
    pub path: String,
    /// Base URL to use for the docs
    #[arg(short, long)]
    pub base_url: Option<String>,
    /// Output directory for the built site
    #[arg(short, long, default_value = "public")]
    pub output_dir: String,
    /// Don't login to gcloud
    #[arg(short, long)]
    pub no_login: bool,
}