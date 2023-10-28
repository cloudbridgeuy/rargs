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
