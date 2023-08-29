use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_render() {
    test_template!(
        "check_dependencies.tera",
        "Equivalent to @dep git",
        serde_json::json!({
            "dep": [{
                "list": ["git"],
            }]
        })
    );
    test_template!(
        "check_dependencies.tera",
        "Equivalent to @dep git Install with: \\e[32mgem install $dependency\\e[0m\\n",
        serde_json::json!({
            "dep": [{
                "list": ["git"],
                "message": "Install with: \\e[32mgem install $dependency\\e[0m\\n",
            }]
        })
    );
    test_template!(
        "check_dependencies.tera",
        "Equivalent to @dep git,curl,make",
        serde_json::json!({
            "dep": [{
                "list": ["git", "curl", "make"],
            }]
        })
    );
    test_template!(
        "check_dependencies.tera",
        "Equivalent to @dep git,curl,make Install with: \\e[32mgem install $dependency\\e[0m\\n",
        serde_json::json!({
            "dep": [{
                "list": ["git", "curl", "make"],
                "message": "Install with: \\e[32mgem install $dependency\\e[0m\\n",
            }]
        })
    );
}
