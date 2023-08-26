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
            "name": "foo",
            "command": {
                "name": "bar",
                "options": {
                    "foo": {
                        "short": "f"
                    },
                    "bar": {
                        "short": "b"
                    },
                    "baz": {}
                }
            }
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with a default value",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "options": {
                    "foo": {
                        "short": "f"
                    },
                    "bar": {
                        "short": "b"
                    },
                    "baz": {}
                }
            },
            "default": "baz",
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with global options",
        serde_json::json!({
            "name": "foo",
            "options": {
                "foo": {
                    "short": "f"
                },
            },
            "command": {
                "name": "bar",
                "options": {
                    "foo": {
                        "short": "f"
                    },
                    "bar": {
                        "short": "b"
                    },
                    "baz": {}
                }
            },
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with a required global option",
        serde_json::json!({
            "name": "foo",
            "options": {
                "foo": {
                    "short": "f",
                    "required": true,
                },
            },
            "command": {
                "name": "bar",
                "options": {
                    "foo": {
                        "short": "f"
                    },
                    "bar": {
                        "short": "b"
                    },
                    "baz": {}
                }
            },
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with a required global option and a default value",
        serde_json::json!({
            "name": "foo",
            "options": {
                "foo": {
                    "short": "f",
                    "required": true,
                },
            },
            "default": "foo",
            "command": {
                "name": "bar",
                "options": {
                    "foo": {
                        "short": "f"
                    },
                    "bar": {
                        "short": "b"
                    },
                    "baz": {}
                }
            },
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands without a required global option and a default value",
        serde_json::json!({
            "name": "foo",
            "options": {
                "foo": {
                    "short": "f",
                },
            },
            "default": "foo",
            "command": {
                "name": "bar",
                "options": {
                    "foo": {
                        "short": "f"
                    },
                    "bar": {
                        "short": "b"
                    },
                    "baz": {}
                }
            },
        })
    );
}
