use tera::Context;
use test_log::test;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "commands": {
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
            "commands": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            },
            "default": "baz",
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "options": {
                "foo": {
                    "short": "f".to_string()
                },
            },
            "commands": {
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
                    "short": "f".to_string(),
                    "required": true,
                },
            },
            "commands": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            }
        })),
    ];

    for object in objects {
        let output = match TEMPLATES.render(
            "parse_command_arguments.tera",
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
