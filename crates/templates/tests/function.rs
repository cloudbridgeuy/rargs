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
}
