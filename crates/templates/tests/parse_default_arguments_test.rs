use tera::Context;
use test_log::test;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![
        Context::from_serialize(serde_json::json!({
            "noname": "foo".to_string(),
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
        })),
        Context::from_serialize(serde_json::json!({
            "noname": "foo".to_string(),
            "rules": ["no-first-option-help"]
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "rules": ["no-first-option-help"]
        })),
        Context::from_serialize(serde_json::json!({
            "noname": "foo".to_string(),
            "rules": ["custom-usage"]
        })),
        Context::from_serialize(serde_json::json!({
            "noname": "foo".to_string(),
            "rules": ["no-first-option-help", "custom-usage"]
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "rules": ["no-first-option-help", "custom-usage"]
        })),
    ];

    for object in objects {
        let output = match TEMPLATES.render(
            "parse_default_arguments.tera",
            &object.expect("Can't create JSON object"),
        ) {
            Ok(o) => o,
            Err(e) => {
                log::error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        insta::assert_snapshot!(output)
    }
}
