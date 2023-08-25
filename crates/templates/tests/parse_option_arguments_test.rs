use tera::Context;
use test_log::test;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "options": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            }
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "flags": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            },
            "rules": ["no-first-option-help"]
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "flags": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            }
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "options": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            },
            "flags": {
                "ffoo": {
                    "short": "F".to_string()
                },
                "fbar": {
                    "short": "B".to_string()
                },
                "fbaz": {}
            }
        })),
    ];

    for object in objects {
        let output = match TEMPLATES.render(
            "parse_option_arguments.tera",
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
