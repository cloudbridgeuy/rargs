use std::fs;

use color_eyre::eyre::Result;

use parser::script::Script;

pub struct Command {
    options: Options,
}

pub struct Options {
    pub script_root: String,
    pub destination: String,
}

impl Command {
    /// Create a new command
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    /// Run the command
    pub fn run(&self) -> Result<()> {
        let source = fs::read_to_string(&self.options.script_root)
            .unwrap_or_else(|_| panic!("Unable to read file: {}", &self.options.script_root));

        let script = Script::from_source(&source)?;
        let name = script.meta.name.to_owned();

        let output = templates::render(&script)?;

        let name = format!(
            "{}/{}.sh",
            self.options.destination,
            name.unwrap_or(self.options.script_root.clone())
        );

        log::info!("Writing script to: {}", &name);
        fs::write(name, output)?;

        Ok(())
    }
}
