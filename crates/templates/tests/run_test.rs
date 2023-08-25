use tera::Context;
use test_log::test;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![
        Context::from_serialize(serde_json::json!({
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            }
        })),
        Context::from_serialize(serde_json::json!({
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            },
            "default": "foo"
        })),
        Context::from_serialize(serde_json::json!({
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            },
            "lines": [
                "API_KEY=\"${API_KEY:-}\"",
                "API_SECRET=\"${API_SECRET:-}\"",
                "echo lorem ipsum",
            ],
        })),
        Context::from_serialize(serde_json::json!({
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            },
            "lines": [
                "API_KEY=\"${API_KEY:-}\"",
                "API_SECRET=\"${API_SECRET:-}\"",
                "echo lorem ipsum",
            ],
            "rules": ["no-first-option-help"]
        })),
    ];

    for object in objects {
        let output = match TEMPLATES.render("run.tera", &object.expect("Can't create JSON object"))
        {
            Ok(o) => o,
            Err(e) => {
                log::error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        insta::assert_snapshot!(output)
    }
}
