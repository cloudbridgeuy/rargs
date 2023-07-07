use std::fs;

use anyhow::Result;

pub struct Command {
    options: Options,
}

pub struct Options {
    pub script_root: String,
}

impl Command {
    /// Create a new command
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    /// Run the command
    pub fn run(&self) -> Result<()> {
        let source = fs::read_to_string(&self.options.script_root)
            .map_err(|e| anyhow::format_err!("Failed to read script root: {}", e))?;
        println!("script root: {}", source);
        Ok(())
    }
}
