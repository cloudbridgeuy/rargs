use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_function_template() {
    test_template!(
        "function.tera",
        "Normal function",
        serde_json::json!({
        "command": {
            "name": "foo",
            "description": "Simple function",
            "lines": ["foo", "bar", "baz"],
        }})
    );
    test_template!(
        "function.tera",
        "Function with no lines",
        serde_json::json!({
            "command": {
                "name": "foo",
                "description": "Simple function",
                "lines": ["foo", "bar", "baz"],
            }
        })
    );
    test_template!(
        "function.tera",
        "Function with no-first-option-help rule",
        serde_json::json!({
        "rules": [
            "no-first-option-help"
        ],
        "command": {
            "name": "foo",
            "description": "Simple function",
        }})
    );

    test_template!(
        "function.tera",
        "Function with custom-usage rule",
        serde_json::json!({
        "rules": [
            "no-first-option-help",
            "custom-usage"
        ],
        "command": {
            "name": "foo",
            "description": "Simple function",
        }})
    );
    test_template!(
        "function.tera",
        "Function with global required option",
        serde_json::json!({
        "options": {
            "require": {
                "name": "require",
                "required": true
            }
        },
        "command": {
            "name": "foo",
            "description": "Function with global required option",
        }})
    );
    test_template!(
        "function.tera",
        "Function with local required option",
        serde_json::json!({
        "command": {
            "name": "foo",
            "description": "Function with a local required option",
                "options": {
                    "require": {
                        "name": "require",
                        "required": true
                    }
                },
            }
        })
    );
    test_template!(
        "function.tera",
        "Function with a local and global required options with the same name",
        serde_json::json!({
        "options": {
            "require": {
                "name": "require",
                "required": true
            }
        },
        "command": {
            "name": "foo",
            "description": "Function with a local required option",
                "options": {
                    "require": {
                        "name": "require",
                        "required": true
                    }
                },
            }
        })
    );
    test_template!(
        "function.tera",
        "Function with a local and two global required options, one with the same name",
        serde_json::json!({
        "options": {
            "foo": {
                "name": "foo",
                "required": true
            },
            "require": {
                "name": "require",
                "required": true
            }
        },
        "command": {
            "name": "foo",
            "description": "Function with a local required option",
                "options": {
                    "require": {
                        "name": "require",
                        "required": true
                    }
                },
            }
        })
    );
    test_template!(
        "function.tera",
        "Function with global default option",
        serde_json::json!({
        "options": {
            "default": {
                "name": "default",
                "default": "foo"
            }
        },
        "command": {
            "name": "foo",
            "description": "Function with global default option",
        }})
    );
    test_template!(
        "function.tera",
        "Function with local default option",
        serde_json::json!({
        "command": {
            "name": "foo",
            "description": "Function with a local default option",
                "options": {
                    "default": {
                        "name": "default",
                        "default": "foo"
                    }
                },
            }
        })
    );
    test_template!(
        "function.tera",
        "Function with a local and global default options with the same name",
        serde_json::json!({
        "options": {
            "default": {
                "name": "default",
                "default": "foo"
            }
        },
        "command": {
            "name": "foo",
            "description": "Function with a local required option",
                "options": {
                    "default": {
                        "name": "default",
                        "default": "foo"
                    }
                },
            }
        })
    );
    test_template!(
        "function.tera",
        "Function with a local and two global default options, one with the same name",
        serde_json::json!({
        "options": {
            "foo": {
                "name": "foo",
                "default": "foo"
            },
            "default": {
                "name": "default",
                "default": true
            }
        },
        "command": {
            "name": "foo",
            "description": "Function with a local default option",
                "options": {
                    "default": {
                        "name": "default",
                        "default": true
                    }
                },
            }
        })
    );
}
