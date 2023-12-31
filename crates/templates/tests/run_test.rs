use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_render() {
    test_template!(
        "run.tera",
        "Run function with multiple commands",
        serde_json::json!({
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            }
        })
    );

    test_template!(
        "run.tera",
        "Run function with multiple commands and a default value",
        serde_json::json!({
            "default": "foo",
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            }
        })
    );

    test_template!(
        "run.tera",
        "Run function with multiple commands and a default value",
        serde_json::json!({
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            },
            "lines": [
                "API_KEY=\"${API_KEY:-}\"",
                "API_SECRET=\"${API_SECRET:-}\"",
                "echo lorem ipsum",
            ],
        })
    );

    test_template!(
        "run.tera",
        "Run function with multiple commands and a default value",
        serde_json::json!({
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            },
            "lines": [
                "API_KEY=\"${API_KEY:-}\"",
                "API_SECRET=\"${API_SECRET:-}\"",
                "echo lorem ipsum",
            ],
            "rules": ["no-first-option-help"]
        })
    );

    test_template!(
        "run.tera",
        "Run function with multiple commands and a default value",
        serde_json::json!({
            "commands": {
                "foo": {},
                "bar": {},
                "baz": {}
            },
            "lines": [
                "API_KEY=\"${API_KEY:-}\"",
                "API_SECRET=\"${API_SECRET:-}\"",
                "echo lorem ipsum",
            ],
            "rules": ["no-force-default"]
        })
    );

    test_template!(
        "run.tera",
        "Run function with a global dependency",
        serde_json::json!({
            "dep": [{
                "list": ["git"],
            }]
        })
    );

    test_template!(
        "run.tera",
        "Run function with a global dependency and a message",
        serde_json::json!({
            "dep": [{
                "list": ["git"],
                "message": "Please install git",
            }]
        })
    );

    test_template!(
        "run.tera",
        "Run function with multiple global dependencies",
        serde_json::json!({
            "dep": [{
                "list": ["git", "curl", "wget"],
            }]
        })
    );

    test_template!(
        "run.tera",
        "Run function with multiple global dependencies and a message",
        serde_json::json!({
            "dep": [{
                "list": ["git", "curl", "wget"],
                "message": "Please install git, curl and wget",
            }]
        })
    );

    test_template!(
        "run.tera",
        "Run function with multiple multiple global dependencies and a message",
        serde_json::json!({
            "dep": [{
                "list": ["git", "curl", "wget"],
                "message": "Please install with apt-get",
            }, {
                "list": ["foo", "bar", "baz"],
                "message": "Please install with apt-get",
            }]
        })
    );

    test_template!(
        "run.tera",
        "Run function with a root command",
        serde_json::json!({
            "root": ["foo", "bar"]
        })
    );

    test_template!(
        "run.tera",
        "Global dependencies are used only for the root command if the use-global-deps-for-root rule is set",
        serde_json::json!({
            "rules": ["use-global-deps-for-root"],
            "dep": [{
                "list": ["git", "curl", "wget"],
                "message": "Please install with apt-get",
            }, {
                "list": ["foo", "bar", "baz"],
                "message": "Please install with apt-get",
            }]
        })
    );
    test_template!(
        "run.tera",
        "Script has an undocumented environment variable",
        serde_json::json!({
            "envs": {
                "FOO": {}
            }
        })
    );
    test_template!(
        "run.tera",
        "Script has an documented environment variable",
        serde_json::json!({
            "envs": {
                "FOO": {
                    "description": "Foo bar baz"
                }
            }
        })
    );
    test_template!(
        "run.tera",
        "Script has a required environment variable",
        serde_json::json!({
            "envs": {
                "FOO": {
                    "required": true
                }
            }
        })
    );
    test_template!(
        "run.tera",
        "Script has a required documented environment variable",
        serde_json::json!({
            "envs": {
                "FOO": {
                    "required": true,
                    "description": "Foo bar baz"
                }
            }
        })
    );
    test_template!(
        "run.tera",
        "Script has an environment variable that should be set on an option",
        serde_json::json!({
            "envs": {
                "FOO": {
                    "option": "foo"
                }
            }
        })
    );
}
