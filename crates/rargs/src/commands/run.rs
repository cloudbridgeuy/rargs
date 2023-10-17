use color_eyre::eyre::Result;

pub struct Command {
    options: Options,
}

#[derive(Debug)]
pub struct Options {
    pub script_root: String,
    pub arguments: Option<Vec<String>>,
}

impl Command {
    /// Creates a new run command
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    /// Runs the command
    pub fn run(&self) -> Result<()> {
        println!("Running script: {}", self.options.script_root);
        if let Some(args) = &self.options.arguments {
            println!("Arguments: {:?}", args);
        }

        Ok(())
    }
}
