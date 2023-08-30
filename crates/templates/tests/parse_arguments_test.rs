use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_render() {
    test_template!(
        "parse_arguments.tera",
        "Parse commands with short and long only options",
        serde_json::json!({
            "name": "foo",
            "commands": {
                "foo": {
                    "short": "f"
                },
                "bar": {
                    "short": "b"
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse commands with a default value",
        serde_json::json!({
            "name": "foo",
            "commands": {
                "foo": {
                    "short": "f"
                },
                "bar": {
                    "short": "b"
                },
                "baz": {}
            },
            "default": "baz",
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse commands with global options",
        serde_json::json!({
            "name": "foo",
            "options": {
                "foo": {
                    "short": "f"
                },
            },
            "commands": {
                "foo": {
                    "short": "f"
                },
                "bar": {
                    "short": "b"
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse commands with a required global option",
        serde_json::json!({
            "name": "foo",
            "options": {
                "foo": {
                    "short": "f",
                    "required": true,
                },
            },
            "commands": {
                "foo": {
                    "short": "f"
                },
                "bar": {
                    "short": "b"
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_arguments.tera",
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
            "commands": {
                "foo": {
                    "short": "f"
                },
                "bar": {
                    "short": "b"
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse commands without a required global option and a default value",
        serde_json::json!({
            "name": "foo",
            "options": {
                "foo": {
                    "short": "f",
                },
            },
            "default": "foo",
            "commands": {
                "foo": {
                    "short": "f"
                },
                "bar": {
                    "short": "b"
                },
                "baz": {}
            }
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse command with multiple global options",
        serde_json::json!({
            "name": "foo",
            "options": {
                "foo": {
                    "short": "f",
                    "multiple": true,
                },
            },
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse command with positional arguments and no commands",
        serde_json::json!({
            "name": "foo",
            "positional_arguments": [{
                "name": "foo",
            }],
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse command with a global option that uses h as short",
        serde_json::json!({
            "name": "foo",
            "options": {
                "host": {
                    "description": "The host to connect to",
                    "short": "h",
                },
            },
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse command with a command option that uses h as short",
        serde_json::json!({
            "name": "foo",
            "commands": {
                "foo": {
                    "short": "f",
                    "options": {
                        "host": {
                            "description": "The host to connect to",
                            "short": "h",
                        },
                    },
                },
            },
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse command with a global option that uses v as short",
        serde_json::json!({
            "name": "foo",
            "version": "0.1.0",
            "flags": {
                "verbose": {
                    "description": "Enable verbose mode",
                    "short": "v"
                },
            },
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse command with aliases",
        serde_json::json!({
            "name": "foo",
            "version": "0.1.0",
            "commands": {
                "foo": {
                    "short": "f",
                    "aliases": ["bar", "baz"],
                    "options": {
                        "host": {
                            "description": "The host to connect to",
                            "short": "h",
                        },
                    },
                },
            },
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse command with the no-force-default rule set",
        serde_json::json!({
            "name": "foo",
            "version": "0.1.0",
            "default": "foo",
            "rules": ["no-force-default"],
            "commands": {
                "foo": {
                    "short": "f",
                    "aliases": ["bar", "baz"],
                    "options": {
                        "host": {
                            "description": "The host to connect to",
                            "short": "h",
                        },
                    },
                },
            },
        })
    );
    test_template!(
        "parse_arguments.tera",
        "Parse arguments with a root command",
        serde_json::json!({
            "name": "foo",
            "version": "0.1.0",
            "root": ["foo", "bar"]
        })
    );
}
