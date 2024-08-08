use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_function_template() {
    test_template!(
        "minimal.tera",
        "Minimal rargs template",
        serde_json::json!({
            "name": "foo",
        })
    );
}

#[test]
fn test_function_flags() {
    test_template!(
        "minimal.tera",
        "Minimal rargs template",
        serde_json::json!({
            "name": "foo",
            "flags": {
                "verbose": {
                    "description": "Enable verbose mode",
                    "short": "v"
                },
            },
            "options": {
                "require": {
                    "name": "require",
                    "required": true
                }
            },
            "lines": [
                "echo \"Hello, world!\"",
                "$sub $@"
            ]
        })
    );
}
