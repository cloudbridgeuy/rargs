use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_render() {
    test_template!(
        "command_usage.tera",
        "Typical Command usage",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with single option",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "options": {
                    "foo": {
                        "short": "f",
                        "name": "foo",
                        "description": "Test foo option"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with required option",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "options": {
                    "required": {
                        "short": "r",
                        "name": "required",
                        "description": "Test required option",
                        "required": "true"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with multiple option",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "options": {
                    "multiple": {
                        "short": "m",
                        "name": "multiple",
                        "description": "Test multiple option",
                        "multiple": "true"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with default option",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "options": {
                    "default": {
                        "short": "d",
                        "name": "default",
                        "description": "Test default option",
                        "default": "foo"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with choices option",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "options": {
                    "choices": {
                        "short": "c",
                        "name": "choices",
                        "description": "Test choices option",
                        "choices": ["foo", "bar", "baz"]
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with required and multiple option",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "options": {
                    "required": {
                        "short": "rm",
                        "name": "required-multiple",
                        "description": "Test required and multiple option",
                        "required": "true",
                        "multiple": "true"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with default and multiple option",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "options": {
                    "default": {
                        "short": "dm",
                        "name": "default-multiple",
                        "description": "Test default and multiple option",
                        "default": "foo",
                        "multiple": "true"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with options and flags",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "flags": {
                    "verbose": {
                        "description": "Test verbose flag"
                    },
                    "short": {
                        "short": "s",
                        "description": "Test short flag"
                    }
                },
                "options": {
                    "choices": {
                        "short": "cm",
                        "name": "choices-multiple",
                        "description": "Test option with choices and multiple",
                        "choices": ["foo", "bar", "baz"],
                        "multiple": "true"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with positional arguments",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "foo",
                    "description": "Test foo positional argument"
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with one positional arguments supporting multiple values",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "foo",
                    "description": "Test foo positional argument",
                    "multiple": true
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with positional arguments with default value",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "foo",
                    "description": "Test foo positional argument",
                    "default": "bar"
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with two positional arguments",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "foo",
                    "description": "Test foo positional argument",
                    "default": "bar",
                    "required": true
                }, {
                    "name": "bar",
                    "description": "Test foo positional argument",
                    "default": "baz",
                    "required": true
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with two positional arguments, the last muliple",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "foo",
                    "description": "Test foo positional argument",
                    "default": "bar",
                    "required": true
                }, {
                    "name": "bar",
                    "description": "Test foo positional argument",
                    "default": "baz",
                    "required": true,
                    "multiple": true
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with a positional argument with a custom value notation",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "source",
                    "description": "Source file",
                    "value_notation": "VALUE_NOTATION"
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with a multiple positional argument with a custom value notation",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "source",
                    "description": "Source file",
                    "multiple": true,
                    "value_notation": "VALUE_NOTATION"
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with a required positional argument with a custom value notation",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "source",
                    "description": "Source file",
                    "required": true,
                    "value_notation": "VALUE_NOTATION"
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with a required multiple positional argument with a custom value notation",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "source",
                    "description": "Source file",
                    "required": true,
                    "multiple": true,
                    "value_notation": "VALUE_NOTATION"
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with a required multiple positional argument with a custom value notation and choices",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "positional_arguments": [{
                    "name": "source",
                    "description": "Source file",
                    "choices": ["foo", "bar", "baz"],
                    "required": true,
                    "multiple": true,
                    "value_notation": "VALUE_NOTATION"
                }]
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with examples",
        serde_json::json!({
        "command": {
            "name": "foo",
            "description": "Test simple usage",
            "examples": [{
                "description": "Verbose mode",
                "command": "-v|--verbose"
            }, {
                "description": "Call command foo in verbose mode",
                "command": "-v|--verbose foo",
            }],
            "flags": {
                "verbose": {
                    "short": "v",
                    "description": "Test verbose flag"
                }
            },
            "options": {
                "choices": {
                    "short": "cm",
                    "name": "choices-multiple",
                    "description": "Test option with choices and multiple",
                    "choices": ["foo", "bar", "baz"],
                    "multiple": "true"
                }
            },
        }})
    );
    test_template!(
        "command_usage.tera",
        "Command usage with an option that uses -h as short",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "options": {
                    "host": {
                        "short": "h",
                        "name": "host",
                        "description": "Test host option"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with a flag that uses -h as short",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "flags": {
                    "height": {
                        "short": "h",
                        "name": "height",
                        "description": "Test height option"
                    }
                }
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with a global flag that uses -h as short",
        serde_json::json!({
            "flags": {
                "height": {
                    "short": "h",
                    "name": "height",
                    "description": "Test height option"
                }
            },
            "command": {
                "name": "foo",
                "description": "Test simple usage",
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with a global option that uses -h as short",
        serde_json::json!({
            "options": {
                "height": {
                    "short": "h",
                    "name": "height",
                    "description": "Test height option"
                }
            },
            "command": {
                "name": "foo",
                "description": "Test simple usage",
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command with aliases",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Test simple usage",
                "aliases": ["bar", "baz"]
            }
        })
    );
}
