use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_render() {
    test_template!(
        "usage.tera",
        "Typical Usage",
        serde_json::json!({
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
        })
    );
    test_template!(
        "usage.tera",
        "Usage with only name and description",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
        })
    );
    test_template!(
        "usage.tera",
        "Usage with single option",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "options": {
                "foo": {
                    "short": "f",
                    "name": "foo",
                    "summary": "Test foo option"
                }
            }
        })
    );
    test_template!(
        "usage.tera",
        "Usage with required option",
        serde_json::json!({
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
        })
    );
    test_template!(
        "usage.tera",
        "Usage with multiple option",
        serde_json::json!({
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
        })
    );
    test_template!(
        "usage.tera",
        "Usage with default option",
        serde_json::json!({
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
        })
    );
    test_template!(
        "usage.tera",
        "Usage with choices option",
        serde_json::json!({
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
        })
    );
    test_template!(
        "usage.tera",
        "Usage with required and multiple option",
        serde_json::json!({
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
        })
    );
    test_template!(
        "usage.tera",
        "Usage with default and multiple option",
        serde_json::json!({
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
        })
    );
    test_template!(
        "usage.tera",
        "Usage with options and flags",
        serde_json::json!({
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
        })
    );
    test_template!(
        "usage.tera",
        "Usage with commands",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "commands": {
                "foo": {
                    "meta": {
                        "description": "Test foo command"
                    }
                }
            }
        })
    );
    test_template!(
        "usage.tera",
        "Usage with commands and flags",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "flags": {
                "verbose": {
                    "summary": "Test verbose flag"
                }
            },
            "commands": {
                "foo": {
                    "meta": {
                        "description": "Test foo command"
                    }
                }
            }
        })
    );
    test_template!(
        "usage.tera",
        "Usage with commands and options",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "options": {
                "choices": {
                    "short": "cm",
                    "name": "choices-multiple",
                    "summary": "Test option with choices and multiple",
                    "choices": ["foo", "bar", "baz"],
                    "multiple": "true"
                }
            },
            "commands": {
                "foo": {
                    "meta": {
                        "description": "Test foo command"
                    }
                }
            }
        })
    );
    test_template!(
        "usage.tera",
        "Usage with commands, flags and options",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "flags": {
                "verbose": {
                    "summary": "Test verbose flag"
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
            },
            "commands": {
                "foo": {
                    "description": "Test foo command"
                }
            }
        })
    );
    test_template!(
        "usage.tera",
        "Usage with extra help message",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "name": "foo".to_string(),
            "options": {
                "required": {
                    "short": "r",
                    "name": "required",
                    "summary": "Test required option",
                    "required": "true"
                }
            }
        })
    );
    test_template!(
        "usage.tera",
        "Usage with option with all flags",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "default": "foo",
            "flags": {
                "verbose": {
                    "summary": "Test verbose flag"
                }
            },
            "commands": {
                "foo": {
                    "description": "Test foo command",
                    "help": "Some additional help message"
                }
            },
            "options": {
                "all": {
                    "short": "a",
                    "name": "all",
                    "summary": "Test option with all attributes",
                    "choices": ["foo", "bar", "baz"],
                    "multiple": "true",
                    "required": "true",
                    "default": "foo"
                }
            }
        })
    );
}
