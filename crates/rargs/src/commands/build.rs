use std::fs;

use color_eyre::eyre::Result;

pub struct Command {
    options: Options,
}

#[derive(Debug)]
pub struct Options {
    pub script: String,
    pub destination: String,
}

impl Command {
    /// Create a new command
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    /// Run the command
    pub fn run(&self) -> Result<()> {
        let scripts =
            super::script::create_script(&self.options.script, &self.options.destination)?;

        for (name, script) in scripts {
            let output = templates::render(&script)?;
            log::info!("Writing script to: {}", &name);
            fs::write(&name, output)?;

            // Try to give the `name` file execution permissions
            super::script::set_permissions(&name)?;
        }

        Ok(())
    }
}
