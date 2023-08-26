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
            "meta": {
                "name": "foo",
                "description": "Simple function"
            },
            "name": "foo",
            "lines": ["foo", "bar", "baz"],
        })
    );
    test_template!(
        "function.tera",
        "Function with no lines",
        serde_json::json!({
            "meta": {
                "name": "function",
                "description": "Test simple function"
            },
            "name": "foo",
        })
    );

    test_template!(
        "function.tera",
        "Function with no-first-option-help rule",
        serde_json::json!({
            "meta": {
                "name": "function",
                "description": "Test simple function"
            },
            "name": "foo",
            "rules": [
                "no-first-option-help"
            ],
        })
    );

    test_template!(
        "function.tera",
        "Function with custom-usage rule",
        serde_json::json!({
            "meta": {
                "name": "function",
                "description": "Test simple function"
            },
            "name": "foo",
            "rules": [
                "no-first-option-help",
                "custom-usage"
            ],
        })
    );
}
