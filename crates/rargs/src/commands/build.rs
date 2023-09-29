use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::{self, Result, WrapErr};

use parser::script::Script;

pub struct Command {
    options: Options,
}

#[derive(Debug)]
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
        // Convert the script_root variable to a PathBuf
        let script_root = fs::canonicalize(&self.options.script_root)?;
        // Check if the destination exists, and create it if it doesn't
        if !PathBuf::from(&self.options.destination).exists() {
            log::info!("Creating destination: {}", &self.options.destination);
            fs::create_dir_all(&self.options.destination)?;
        }
        let destination = fs::canonicalize(&self.options.destination)?;

        // Check if the script_root file exists
        if !script_root.exists() {
            return Err(eyre::format_err!(
                "Unable to find script: {}",
                &self.options.script_root
            ));
        }

        // Get the script_root parent dir absolute path
        let script_root_parent = match script_root.parent() {
            Some(path) => path,
            None => return Err(eyre::format_err!("Unable to get parent dir of script_root")),
        };

        let source = fs::read_to_string(&script_root).wrap_err(format!(
            "Unable to read file: {}",
            &self.options.script_root
        ))?;

        let mut script = Script::from_source(&source)?;

        for command in script.commands.values_mut() {
            if let Some(subcommand) = &command.subcommand {
                let subcommand = PathBuf::from(subcommand);
                log::info!("Subcommand: {}", subcommand.display());
                let subcommand_destination = format!(
                    "{}/{}",
                    &destination.display(),
                    subcommand
                        .parent()
                        .ok_or_else(|| {
                            eyre::format_err!(
                                "Unable to get parent dir of subcommand: {}",
                                subcommand.display()
                            )
                        })?
                        .display()
                )
                .replace("/./", "/");
                log::info!("Subcommand destination: {}", subcommand_destination);
                let subcommand = fs::canonicalize(PathBuf::from(format!(
                    "{}/{}",
                    script_root_parent.display(),
                    subcommand.display(),
                )))?;
                log::info!("Absolute subcommand path: {}", subcommand.display());
                let options = Options {
                    script_root: subcommand.display().to_string(),
                    destination: subcommand_destination.to_string(),
                };
                log::info!("Creating new command with options: {:?}", &options);
                Self::new(options).run()?;
                command.subcommand = Some(format!(
                    "{}/{}",
                    subcommand_destination,
                    subcommand.file_name().unwrap().to_str().unwrap()
                ));
            }
        }

        let output = templates::render(&script)?;

        let name = format!(
            "{}/{}",
            self.options.destination,
            script_root
                .file_name()
                .ok_or_else(|| eyre::format_err!("Unable to get script_root filename"))?
                .to_str()
                .ok_or_else(|| eyre::format_err!(
                    "Unable to convert script_root filename to str"
                ))?
        );

        log::info!("Writing script to: {}", &name);
        fs::write(&name, output)?;

        // Try to give the `name` file execution permissions
        #[cfg(unix)]
        {
            log::info!("Setting permissions on: {}", &name);
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&name)?.permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&name, permissions)?;
        }

        Ok(())
    }
}
