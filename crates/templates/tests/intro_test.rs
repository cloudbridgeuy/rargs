use tera::Context;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![Context::from_serialize(serde_json::json!({
        "rargc_version": env!("CARGO_PKG_VERSION"),
    }))];

    for object in objects {
        let output =
            match TEMPLATES.render("intro.tera", &object.expect("Can't create JSON object")) {
                Ok(o) => o,
                Err(e) => {
                    log::error!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

        println!("output: {:#?}", output);

        insta::assert_snapshot!(output)
    }
}
