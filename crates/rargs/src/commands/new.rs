use color_eyre::eyre::Result;
use std::fs;
use std::path::PathBuf;

pub struct Command {
    options: Options,
}

#[derive(Debug, serde::Serialize)]
pub struct Options {
    pub destination: String,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
}

impl Command {
    /// Creates a new command
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    /// Runs the command
    pub fn run(&self) -> Result<()> {
        // Check if the destination exists, and create it if it doesn't
        if !PathBuf::from(&self.options.destination).exists() {
            log::info!("Creating destination: {}", &self.options.destination);
            fs::create_dir_all(&self.options.destination)?;
        }

        let name = format!(
            "{}/{}.sh",
            &self.options.destination,
            &self.options.name.replace('-', "_")
        );

        let output = templates::minimal(&self.options)?;

        fs::write(name, output)?;

        Ok(())
    }
}
