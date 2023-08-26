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

        let mut script = Script::from_source(&source)?;

        // If the script has a name, use that, otherwise use the name of the file
        let mut name = script
            .name
            .to_owned()
            .unwrap_or(self.options.script_root.clone());
        if name.ends_with(".sh") {
            name.truncate(name.len() - 3);
            name = name.clone().split('/').last().unwrap().to_string();
        }

        script.name = Some(name.clone());

        let output = templates::render(&script)?;

        let name = format!("{}/{}.sh", self.options.destination, name);

        log::info!("Writing script to: {}", &name);
        fs::write(name, output)?;

        Ok(())
    }
}
