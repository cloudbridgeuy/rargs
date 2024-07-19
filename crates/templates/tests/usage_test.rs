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
                    "description": "Test required option",
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
                    "description": "Test foo option"
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
                    "description": "Test required option",
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
                    "description": "Test multiple option",
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
                    "description": "Test default option",
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
                    "description": "Test choices option",
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
                    "description": "Test required and multiple option",
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
                    "description": "Test default and multiple option",
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
                    "description": "Test foo command"
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
                    "description": "Test verbose flag"
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
        "Usage with commands and options",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "options": {
                "choices": {
                    "short": "cm",
                    "name": "choices-multiple",
                    "description": "Test option with choices and multiple",
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
        "Usage with commands, flags and options",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "flags": {
                "verbose": {
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
                    "description": "Test required option",
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
                    "description": "Test verbose flag"
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
                    "description": "Test option with all attributes",
                    "choices": ["foo", "bar", "baz"],
                    "multiple": "true",
                    "required": "true",
                    "default": "foo"
                }
            }
        })
    );
    test_template!(
        "usage.tera",
        "Usage with multiple commands with default",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "default": "foo",
            "flags": {
                "verbose": {
                    "description": "Test verbose flag"
                }
            },
            "commands": {
                "foo": {
                    "description": "Test foo command",
                    "help": "Some additional help message"
                },
                "bar": {
                    "description": "Test bar command",
                    "help": "Some additional help message"
                },
                "baz": {
                    "description": "Test baz command",
                    "help": "Some additional help message"
                }
            },
        })
    );
    test_template!(
        "usage.tera",
        "Usage with multiple commands",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "flags": {
                "verbose": {
                    "description": "Test verbose flag"
                }
            },
            "commands": {
                "foo": {
                    "description": "Test foo command",
                    "help": "Some additional help message"
                },
                "bar": {
                    "description": "Test bar command",
                    "help": "Some additional help message"
                },
                "baz": {
                    "description": "Test baz command",
                    "help": "Some additional help message"
                }
            },
        })
    );
    test_template!(
        "usage.tera",
        "Usage with examples",
        serde_json::json!({
            "name": "usage",
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
            "commands": {
                "foo": {
                    "description": "Test foo command",
                    "help": "Some additional help message"
                },
                "bar": {
                    "description": "Test bar command",
                    "help": "Some additional help message"
                },
                "baz": {
                    "description": "Test baz command",
                    "help": "Some additional help message"
                }
            },
        })
    );
    test_template!(
        "usage.tera",
        "Usage with positional arguments",
        serde_json::json!({
            "name": "usage",
            "description": "Test usage with positional arguments",
            "positional_arguments": [{
                "name": "foo",
                "description": "Test foo command"
            }, {
                "name": "bar",
                "description": "Test bar command"
            }]
        })
    );

    test_template!(
        "usage.tera",
        "Usage with positional arguments",
        serde_json::json!({
            "name": "usage",
            "description": "Test usage with positional arguments, one of them required",
            "positional_arguments": [{
                "name": "foo",
                "description": "Test foo command",
                "required": true
            }, {
                "name": "bar",
                "description": "Test bar command"
            }]
        })
    );

    test_template!(
        "usage.tera",
        "Usage with a global option that uses -h as short",
        serde_json::json!({
            "name": "usage",
            "description": "Test usage with a global option that uses -h as short",
            "options": {
                "host": {
                    "short": "h",
                    "name": "host",
                    "description": "Test host option"
                }
            }
        })
    );

    test_template!(
        "usage.tera",
        "Usage with a global flag that uses -h as short",
        serde_json::json!({
            "name": "usage",
            "description": "Test usage with a global option that uses -h as short",
            "flags": {
                "host": {
                    "short": "h",
                    "name": "host",
                    "description": "Test host option"
                }
            }
        })
    );

    test_template!(
        "usage.tera",
        "Usage with a global option that uses -v as short",
        serde_json::json!({
            "name": "usage",
            "version": "0.0.1",
            "description": "Test usage with a global option that uses -v as short",
            "options": {
                "verbose": {
                    "short": "v",
                    "name": "host",
                    "description": "Test verbose option"
                }
            }
        })
    );

    test_template!(
        "usage.tera",
        "Usage with a global flag that uses -v as short",
        serde_json::json!({
            "name": "usage",
            "version": "0.0.1",
            "description": "Test usage with a global option that uses -v as short",
            "flags": {
                "verbose": {
                    "short": "v",
                    "name": "host",
                    "description": "Test verbose option"
                }
            }
        })
    );

    test_template!(
        "usage.tera",
        "Usage with option with all flags and value notations",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "default": "foo",
            "flags": {
                "verbose": {
                    "description": "Test verbose flag"
                }
            },
            "commands": {
                "foo": {
                    "description": "Test foo command",
                    "help": "Some additional help message"
                }
            },
            "positional_arguments": [{
                "name": "some",
                "description": "Test argument with all attributes",
                "choices": ["foo", "bar", "baz"],
                "multiple": "true",
                "required": "true",
                "default": "foo",
                "value_notation": "ELSE"
            }],
            "options": {
                "all": {
                    "short": "a",
                    "name": "all",
                    "description": "Test option with all attributes",
                    "choices": ["foo", "bar", "baz"],
                    "multiple": "true",
                    "required": "true",
                    "default": "foo",
                    "value_notation": "ELSE"
                }
            }
        })
    );

    test_template!(
        "usage.tera",
        "Usage with option with all flags and value notations",
        serde_json::json!({
            "name": "usage",
            "description": "Test simple usage",
            "default": "foo",
            "flags": {
                "verbose": {
                    "description": "Test verbose flag"
                }
            },
            "commands": {
                "foo": {
                    "description": "Test foo command",
                    "help": "Some additional help message",
                    "positional_arguments": [{
                        "name": "some",
                        "description": "Test argument with all attributes",
                        "choices": ["foo", "bar", "baz"],
                        "multiple": "true",
                        "required": "true",
                        "default": "foo",
                        "value_notation": "ELSE"
                    }],
                    "options": {
                        "all": {
                            "short": "a",
                            "name": "all",
                            "description": "Test option with all attributes",
                            "choices": ["foo", "bar", "baz"],
                            "multiple": "true",
                            "required": "true",
                            "default": "foo",
                            "value_notation": "ELSE"
                        }
                    }
                }
            },
        })
    );
}
