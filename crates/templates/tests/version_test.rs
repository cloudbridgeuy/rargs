use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_render() {
    test_template!(
        "version.tera",
        "With version",
        serde_json::json!({
            "version": "0.0.1".to_string(),
        })
    );
    test_template!("version.tera", "Without version", serde_json::json!({}));
}
