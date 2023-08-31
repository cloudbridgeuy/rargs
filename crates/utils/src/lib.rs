pub extern crate paste;

#[macro_export]
macro_rules! test_template {
    ($template:expr, $description:expr, $json:expr) => {
        let object = Context::from_serialize($json);

        let output =
            match TEMPLATES.render($template, &object.expect("Can't create JSON object")) {
                Ok(o) => o,
                Err(e) => {
                    log::error!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

        insta::with_settings!({
            description => $description,
        }, {
            insta::assert_snapshot!(output)
        });
    }
}

#[macro_export]
macro_rules! test_script {
    ($script:expr, $stream:expr, $options:expr) => {
        let script_name = $script.to_string();
        let options = $options.to_string();
        let stream = $stream.to_string();
        let description = format!("[{stream}] {script_name} {options}");
        let output_dir = "./examples/output".to_string();
        let src = format!("./examples/{script_name}");
        let dst = format!("./examples/output/{script_name}");

        log::info!("{}", description);
        let output = Command::new(&dst)
            .args(options.split(" ").collect::<Vec<&str>>())
            .output()
            .unwrap_or_else(|e| panic!("failed to execute the '{dst}' script with error: {e}"));
        insta::with_settings!({
            description => description,
        }, {
            insta::assert_snapshot!(String::from_utf8(match $stream {
                "stdout" => output.stdout,
                "stderr" => output.stderr,
                _ => panic!("invalid stream"),
            }).unwrap_or_else(|e| panic!("failed to execute the '{dst}' script with error: {e}")))
        });
    };
}

#[macro_export]
macro_rules! build_script {
    ($script:expr) => {
        let script_name = $script.to_string();
        let output_dir = "./examples/output".to_string();
        let src = format!("./examples/{script_name}");
        let dst = format!("./examples/output/{script_name}");

        log::info!("Build ./examples/output/minimal.rs");
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "rargc",
                "--release",
                "--",
                "build",
                "-d",
                &output_dir,
                &src,
            ])
            .output()
            .unwrap_or_else(|e| panic!("failed to build rargc script with error: {e}"));

        println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
        println!("stderr: {}", String::from_utf8(output.stderr).unwrap());

        log::info!("Check that the {dst} file exists");
        assert!(std::path::Path::new(&dst).exists());

        // Read the `dst` file into a variable of type `&str`
        let dst = std::fs::read_to_string(&dst)
            .unwrap_or_else(|e| panic!("failed to read the '{dst}' script with error: {e}"));

        insta::with_settings!({
            description => script_name,
        }, {
            insta::assert_snapshot!(dst)
        });
    };
}
