use color_eyre::eyre::Result;
use std::fs;
use std::process::Command as ProcessCommand;
use tempfile::tempdir;

pub struct Command {
    options: Options,
}

#[derive(Debug)]
pub struct Options {
    pub script: String,
    pub arguments: Vec<String>,
}

impl Command {
    /// Creates a new run command
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    /// Runs the command
    pub fn run(&self) -> Result<()> {
        let tmp = tempdir()?;
        let tmp_dir = tmp.path().to_string_lossy().to_string();
        let tmp_script = "main.sh".to_string();

        let root_path = format!("{}/{}", &tmp_dir, &tmp_script);
        // Store the contents of self.options.script inside `root_path`.
        fs::write(&root_path, &self.options.script)?;

        let root = fs::canonicalize(&root_path)?;

        super::build::Command::new(super::build::Options {
            script: root_path,
            destination: tmp_dir,
        })
        .run()?;

        let args = self
            .options
            .arguments
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();

        let output = ProcessCommand::new(root).args(args).output()?;

        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        tmp.close()?;

        Ok(())
    }
}
