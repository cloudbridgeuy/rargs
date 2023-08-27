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
                "name": "usage",
                "description": "Test simple usage",
            }
        })
    );
    test_template!(
        "command_usage.tera",
        "Command usage with single option",
        serde_json::json!({
            "command": {
                "name": "usage",
                "description": "Test simple usage",
                "options": {
                    "foo": {
                        "short": "f",
                        "name": "foo",
                        "summary": "Test foo option"
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
                "name": "usage",
                "description": "Test simple usage",
                "options": {
                    "required": {
                        "short": "r",
                        "name": "required",
                        "summary": "Test required option",
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
                "name": "usage",
                "description": "Test simple usage",
                "options": {
                    "multiple": {
                        "short": "m",
                        "name": "multiple",
                        "summary": "Test multiple option",
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
                "name": "usage",
                "description": "Test simple usage",
                "options": {
                    "default": {
                        "short": "d",
                        "name": "default",
                        "summary": "Test default option",
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
                "name": "usage",
                "description": "Test simple usage",
                "options": {
                    "choices": {
                        "short": "c",
                        "name": "choices",
                        "summary": "Test choices option",
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
                "name": "usage",
                "description": "Test simple usage",
                "options": {
                    "required": {
                        "short": "rm",
                        "name": "required-multiple",
                        "summary": "Test required and multiple option",
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
                "name": "usage",
                "description": "Test simple usage",
                "options": {
                    "default": {
                        "short": "dm",
                        "name": "default-multiple",
                        "summary": "Test default and multiple option",
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
                "name": "usage",
                "description": "Test simple usage",
                "flags": {
                    "verbose": {
                        "summary": "Test verbose flag"
                    },
                    "short": {
                        "short": "s",
                        "summary": "Test short flag"
                    }
                },
                "options": {
                    "choices": {
                        "short": "cm",
                        "name": "choices-multiple",
                        "summary": "Test option with choices and multiple",
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
                "name": "usage",
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
                "name": "usage",
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
                "name": "usage",
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
                "name": "usage",
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
                "name": "usage",
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
                "name": "usage",
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
                "name": "usage",
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
                "name": "usage",
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
                "name": "usage",
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
}
