use color_eyre::eyre::Result;

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
        println!(
            "Running tree command for script root: {}",
            self.options.script_root
        );
        Ok(())
    }
}
