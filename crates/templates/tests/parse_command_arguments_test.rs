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
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with one positional arguments",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "positional_arguments": [{
                    "name": "foo",
                }]
            },
        })
    );
    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with one required positional arguments",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "positional_arguments": [{
                    "name": "foo",
                    "required": true
                }]
            },
        })
    );

    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with two positional arguments",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "positional_arguments": [{
                    "name": "foo",
                    "required": true
                }, {
                    "name": "bar",
                    "required": false
                }]
            },
        })
    );

    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with two required positional arguments",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "positional_arguments": [{
                    "name": "foo",
                    "required": true
                }, {
                    "name": "bar",
                    "required": true
                }]
            },
        })
    );

    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with three positional arguments",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "positional_arguments": [{
                    "name": "foo",
                    "required": true
                }, {
                    "name": "bar",
                    "required": false
                }, {
                    "name": "baz",
                    "required": false
                }]
            },
        })
    );

    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with three positional arguments, two of them required",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "positional_arguments": [{
                    "name": "foo",
                    "required": true
                }, {
                    "name": "bar",
                    "required": true
                }, {
                    "name": "baz",
                    "required": false
                }]
            },
        })
    );

    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with three positional arguments, three of them required",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "positional_arguments": [{
                    "name": "foo",
                    "required": true
                }, {
                    "name": "bar",
                    "required": true
                }, {
                    "name": "baz",
                    "required": true
                }]
            },
        })
    );

    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with three positional arguments, three of them required, the last one multiple",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "bar",
                "positional_arguments": [{
                    "name": "foo",
                    "required": true
                }, {
                    "name": "bar",
                    "required": true
                }, {
                    "name": "baz",
                    "required": true,
                    "multiple": true
                }]
            },
        })
    );

    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with one multiple positional arguments",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "foo",
                "positional_arguments": [{
                    "name": "foo",
                    "required": true,
                    "multiple": true
                }]
            },
        })
    );

    test_template!(
        "parse_command_arguments.tera",
        "Parse commands with one multiple option",
        serde_json::json!({
            "name": "foo",
            "command": {
                "name": "foo",
                "options": {
                    "multiple": {
                        "name": "foo",
                        "required": true,
                        "multiple": true
                    }
                }
            },
        })
    );
}
