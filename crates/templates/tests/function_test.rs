use tera::Context;
use test_log::test;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "function",
                "description": "Test simple function"
            },
            "name": "foo".to_string(),
            "lines": [
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string()
            ]
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "function",
                "description": "Test simple function"
            },
            "name": "foo".to_string(),
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "function",
                "description": "Test simple function"
            },
            "name": "foo".to_string(),
            "rules": [
                "no-first-option-help"
            ],
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "function",
                "description": "Test simple function"
            },
            "name": "foo".to_string(),
            "rules": [
                "no-first-option-help",
                "custom-usage"
            ],
        })),
    ];

    for object in objects {
        let output =
            match TEMPLATES.render("function.tera", &object.expect("Can't create JSON object")) {
                Ok(o) => o,
                Err(e) => {
                    log::error!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

        insta::assert_snapshot!(output)
    }
}
