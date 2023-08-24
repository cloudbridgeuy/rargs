use tera::Context;
use test_log::test;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "name": "foo".to_string(),
            "options": {
                "required": {
                    "short": "r",
                    "name": "required",
                    "summary": "Test required option",
                    "required": "true"
                }
            }
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "options": {
                "foo": {
                    "short": "f",
                    "name": "foo",
                    "summary": "Test foo option"
                }
            }
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "options": {
                "required": {
                    "short": "r",
                    "name": "required",
                    "summary": "Test required option",
                    "required": "true"
                }
            }
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "options": {
                "multiple": {
                    "short": "m",
                    "name": "multiple",
                    "summary": "Test multiple option",
                    "multiple": "true"
                }
            }
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "options": {
                "default": {
                    "short": "d",
                    "name": "default",
                    "summary": "Test default option",
                    "default": "foo"
                }
            }
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "options": {
                "choices": {
                    "short": "c",
                    "name": "choices",
                    "summary": "Test option with choices",
                    "choices": ["foo", "bar", "baz"]
                }
            }
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "options": {
                "required": {
                    "short": "rm",
                    "name": "required-multiple",
                    "summary": "Test required and multiple option",
                    "required": "true",
                    "multiple": "true"
                }
            }
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "options": {
                "default": {
                    "short": "dm",
                    "name": "default-multiple",
                    "summary": "Test default and multiple option",
                    "default": "foo",
                    "multiple": "true"
                }
            }
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
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
        })),
        Context::from_serialize(serde_json::json!({
            "meta": {
                "name": "usage",
                "description": "Test simple usage"
            },
            "flags": {
                "verbose": {
                    "summary": "Test verbose flag"
                }
            },
            "commands": {
                "foo": {
                    "meta": {
                        "description": "Test foo command",
                        "help": "Something\n      With\n      Multiple\n      Lines"
                    }
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
        })),
    ];

    for object in objects {
        let output =
            match TEMPLATES.render("usage.tera", &object.expect("Can't create JSON object")) {
                Ok(o) => o,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

        insta::assert_snapshot!(output)
    }
}
