use crate::cli;
use color_eyre::eyre::Result;
use duct::cmd;

// Implement the From trait for the cli::DeployDocsArgs struct so that we can convert it into a
// scripts::BuildDocsArgs struct
impl From<&cli::DeployDocsArgs> for cli::BuildDocsArgs {
    fn from(args: &cli::DeployDocsArgs) -> Self {
        cli::BuildDocsArgs {
            path: args.path.clone(),
            base_url: args
                .base_url
                .as_ref()
                .unwrap_or(&"rargs.cloudbridge.uy".to_owned())
                .to_string(),
            output_dir: args.output_dir.clone(),
        }
    }
}

/// Build the documentation site
pub fn build_docs(args: &cli::BuildDocsArgs) -> Result<()> {
    let previous_dir = std::env::current_dir()?;
    println!("Change into the documentation directory");
    std::env::set_current_dir(&args.path)?;
    println!("Run Zola to build the documentation site");
    cmd!(
        "zola",
        "build",
        "--base-url",
        format!("https://{}", &args.base_url),
        "--output-dir",
        &args.output_dir,
        "--force",
    )
    .run()?;
    // Return to the previous working directory.
    std::env::set_current_dir(previous_dir)?;

    Ok(())
}

/// Runs the zola development server
pub fn doc_dev() -> Result<()> {
    println!("Running the zola development server");
    cmd!("zola", "serve").dir("./web").run()?;
    Ok(())
}

/// Create the release binaries for all the supported architectures.
pub fn release(args: &cli::ReleaseArgs) -> Result<()> {
    println!("Building release binaries");
    if !args.no_apple_x86_64 {
        println!("Building for x86_64");
        cmd!(
            "cargo",
            "build",
            "--bin",
            &args.binary,
            "--target",
            "x86_64-apple-darwin",
            "--release"
        )
        .run()?;
    }

    if !args.no_apple_silicon {
        println!("Building for Apple Silicon");
        cmd!(
            "cargo",
            "build",
            "--bin",
            &args.binary,
            "--target",
            "aarch64-apple-darwin",
            "--release"
        )
        .run()?;
    }

    if !args.no_linux_aarch64 {
        // println!("Building for x86_64 Linux");
        // cmd!(
        //     "cargo",
        //     "build",
        //     "--bin",
        //     &args.binary,
        //     "--target",
        //     "aarch64-unknown-linux-gnu"
        // )
        // .run()?;
        println!("[WIP] Building for AAarch64 Linux");
    }

    Ok(())
}

pub fn install(args: &cli::InstallArgs) -> Result<()> {
    release(&cli::ReleaseArgs {
        binary: args.name.clone(),
        no_apple_x86_64: true,
        no_apple_silicon: false,
        no_linux_aarch64: true,
    })?;

    let target_path = "target/aarch64-apple-darwin/release/".to_string() + &args.name;

    cmd!("cp", &target_path, &args.path).run()?;
    cmd!("chmod", "+x", &args.path).run()?;

    Ok(())
}

/// Deploy the latest documentation
pub fn deploy_docs(args: &cli::DeployDocsArgs) -> Result<()> {
    println!("Building docs");
    build_docs(&args.into())?;

    if !args.no_login {
        println!("Authenticating with gcloud");
        cmd!(
            "gcloud",
            "config",
            "configurations",
            "activate",
            &args.gcp_config
        )
        .run()?;
        cmd!("gcloud", "auth", "login").run()?;
        cmd!("gcloud", "auth", "application-default", "login").run()?;
        cmd!(
            "gcloud",
            "config",
            "configurations",
            "describe",
            &args.gcp_config
        )
        .run()?;
    }

    println!(
        "Uploading files located at {}/{} to bucket: {}",
        &args.path, &args.output_dir, &args.url
    );
    println!("{}", cmd!("pwd").read()?);
    cmd!(
        "gcloud",
        "storage",
        "rsync",
        &format!("{}/{}", &args.path, &args.output_dir),
        &format!("gs://{}", &args.url),
        "--recursive",
        "--delete-unmatched-destination-objects"
    )
    .run()?;

    println!("Setting the default ACL for the bucket");
    cmd!(
        "gcloud",
        "storage",
        "buckets",
        "add-iam-policy-binding",
        &format!("gs://{}", &args.url),
        "--member=allUsers",
        "--role=roles/storage.objectViewer"
    )
    .run()?;

    println!("Assigning specialty pages");
    cmd!(
        "gcloud",
        "storage",
        "buckets",
        "update",
        &format!("gs://{}", &args.url),
        "--web-main-page-suffix=index.html",
        "--web-error-page=404.html",
    )
    .run()?;

    Ok(())
}
