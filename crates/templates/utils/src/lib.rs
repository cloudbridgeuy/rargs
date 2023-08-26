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
