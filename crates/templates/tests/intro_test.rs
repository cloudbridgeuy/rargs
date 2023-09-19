use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_render() {
    test_template!(
        "intro.tera",
        "Get the rargs_version from the CARGO_PKG_VERSION environment variable",
        serde_json::json!({
            "rargs_version": env!("CARGO_PKG_VERSION"),
        })
    );
}
