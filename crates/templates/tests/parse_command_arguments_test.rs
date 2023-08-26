use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_render() {
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with short and long only options",
        serde_json::json!({
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
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with a default value",
        serde_json::json!({
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
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with global options",
        serde_json::json!({
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
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with a required global option",
        serde_json::json!({
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
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with a required global option and a default value",
        serde_json::json!({
            "name": "foo".to_string(),
            "options": {
                "foo": {
                    "short": "f".to_string(),
                    "required": true,
                },
            },
            "default": "foo",
            "commands": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands without a required global option and a default value",
        serde_json::json!({
            "name": "foo".to_string(),
            "options": {
                "foo": {
                    "short": "f".to_string(),
                },
            },
            "default": "foo",
            "commands": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse unnamed commands with short and long only options",
        serde_json::json!({
            "commands": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse unnamed commands with a default value",
        serde_json::json!({
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
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse unnamed commands with global options",
        serde_json::json!({
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
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse unnamed commands with a required global option",
        serde_json::json!({
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
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse unnamed commands with a required global option and a default value",
        serde_json::json!({
            "options": {
                "foo": {
                    "short": "f".to_string(),
                    "required": true,
                },
            },
            "default": "foo",
            "commands": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse unnamed commands without a required global option and a default value",
        serde_json::json!({
            "options": {
                "foo": {
                    "short": "f".to_string(),
                },
            },
            "default": "foo",
            "commands": {
                "foo": {
                    "short": "f".to_string()
                },
                "bar": {
                    "short": "b".to_string()
                },
                "baz": {}
            }
        })
    );
}
