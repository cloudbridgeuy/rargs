use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::{self, Result, WrapErr};

use parser::script::Script;

/// Try to give the `name` file execution permissions
pub fn set_permissions(name: &str) -> Result<()> {
    #[cfg(unix)]
    {
        log::info!("Setting permissions on: {}", &name);
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(name)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(name, permissions)?;
    }
    Ok(())
}

/// Create a script and all its child scripts from a root script path
pub fn create_script(script_root: &str, destination: &str) -> Result<Vec<(String, Script)>> {
    // Convert the script_root variable to a PathBuf
    let root = fs::canonicalize(script_root)?;
    // Check if the destination exists, and create it if it doesn't
    if !PathBuf::from(destination).exists() {
        log::info!("Creating destination: {}", destination);
        fs::create_dir_all(destination)?;
    }
    let dest = fs::canonicalize(destination)?;

    // Check if the script_root file exists
    if !root.exists() {
        return Err(eyre::format_err!("Unable to find script: {}", script_root));
    }

    // Get the script_root parent dir absolute path
    let script_root_parent = match root.parent() {
        Some(path) => path,
        None => return Err(eyre::format_err!("Unable to get parent dir of script_root")),
    };

    let source =
        fs::read_to_string(&root).wrap_err(format!("Unable to read file: {}", script_root))?;

    let mut script = Script::from_source(&source)?;

    let mut scripts: Vec<(String, Script)> = vec![];

    for command in script.commands.values_mut() {
        if let Some(subcommand) = &command.subcommand {
            let subcommand = PathBuf::from(subcommand);
            log::info!("Subcommand: {}", subcommand.display());
            let subcommand_destination = format!(
                "{}/{}",
                &dest.display(),
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

            let mut sub_script =
                create_script(&subcommand.display().to_string(), &subcommand_destination)?;

            scripts.append(&mut sub_script);

            command.subcommand = Some(format!(
                "{}/{}",
                subcommand_destination,
                subcommand.file_name().unwrap().to_str().unwrap()
            ));
        }
    }

    let name = format!(
        "{}/{}",
        destination,
        root.file_name()
            .ok_or_else(|| eyre::format_err!("Unable to get script_root filename"))?
            .to_str()
            .ok_or_else(|| eyre::format_err!("Unable to convert script_root filename to str"))?
    );

    scripts.push((name, script));

    Ok(scripts)
}
