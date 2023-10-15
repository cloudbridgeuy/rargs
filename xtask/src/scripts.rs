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
