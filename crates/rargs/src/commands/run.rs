use color_eyre::eyre::{self, Result};
use std::fs;
use std::process::Command as ProcessCommand;
use tempfile::tempdir;

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
        // Create a temporary directory to store the scripts
        let temp_dir = tempdir()?;

        // Build the scripts
        super::build::Command::new(super::build::Options {
            script_root: self.options.script_root.clone(),
            destination: temp_dir.path().to_string_lossy().to_string(),
        })
        .run()?;

        // Convert the script_root variable to a PathBuf
        let root = fs::canonicalize(&self.options.script_root)?;

        // Get the root script filename, and convert it to the build script inside the tmp
        // directory
        let root_script = root
            .file_name()
            .ok_or_else(|| eyre::format_err!("Unable to get root script filename"))?
            .to_string_lossy()
            .to_string();

        // Run the root_script, redirecting `stdout` and `stderr` to the current process.
        // Regardless of the status code of the root_script, we want to continue running the
        // process and clean up the temporary directory.
        let mut args: Vec<&str> = vec![];

        if let Some(arguments) = &self.options.arguments {
            args = arguments.iter().map(AsRef::as_ref).collect();
        }

        let output = ProcessCommand::new(format!(
            "{}/{}",
            temp_dir.path().to_string_lossy(),
            root_script
        ))
        .args(args)
        .output()?;

        // Handle the output
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        // Remove the tmp directory
        temp_dir.close()?;

        Ok(())
    }
}
