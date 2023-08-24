use tera::Context;
use test_log::test;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![Context::from_serialize(serde_json::json!({
        "version": "0.0.1".to_string(),
    }))];

    for object in objects {
        let output =
            match TEMPLATES.render("version.tera", &object.expect("Can't create JSON object")) {
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
