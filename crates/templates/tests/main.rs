use tera::Context;
use test_log::test;

use templates::TEMPLATES;
use utils::test_template;

#[test]
fn test_function_template() {
    test_template!(
        "main.tera",
        "Script with error",
        serde_json::json!({
            "commands": {
            "upload": {
                "aliases": ["u", "push"],
                "description": "Upload a file",
                "examples": [
                {
                    "description": "Upload a file to the internet",
                    "command": "upload example.com"
                },
                {
                    "description": "Upload a file and use a different alias",
                    "command": "push example.com"
                }
                ],
                "lines": ["\techo \"${rargs_input[*]}\""],
                "name": "upload",
                "options": {
                "password": {
                    "name": "password",
                    "description": "Password to use for logging in",
                    "short": "p",
                    "multiple": false,
                    "required": false
                },
                "user": {
                    "name": "user",
                    "description": "Username to use for logging in",
                    "short": "u",
                    "multiple": false,
                    "required": false
                }
                },
                "positional_arguments": [
                {
                    "name": "source",
                    "description": "URL to download from",
                    "required": true,
                    "multiple": false
                }
                ],
                "private": false
            },
            "download": {
                "aliases": ["d", "down"],
                "description": "Download a file",
                "examples": [
                {
                    "description": "Download a file from the internet",
                    "command": "download example.com"
                },
                {
                    "description": "Download a file and use a different alias",
                    "command": "down example.com"
                }
                ],
                "flags": {
                "force": {
                    "name": "force",
                    "description": "Overwrite existing files",
                    "short": "f",
                    "multiple": false
                }
                },
                "lines": ["\techo \"${rargs_input[*]}\""],
                "name": "download",
                "positional_arguments": [
                {
                    "name": "source",
                    "description": "URL to download from",
                    "required": true,
                    "multiple": false
                },
                {
                    "name": "target",
                    "description": "Target filename (default: same as source)",
                    "required": false,
                    "multiple": false
                }
                ],
                "private": false
            }
            },
            "description": "Sample application",
            "examples": [
            {
                "description": "Download command help",
                "command": "download -h"
            },
            {
                "description": "Download command example",
                "command": "download [OPTIONS] SOURCE [TARGET]"
            },
            {
                "description": "Upload command help",
                "command": "upload -h"
            },
            {
                "description": "Upload command example",
                "command": "upload [OPTIONS] SOURCE"
            }
            ],
            "name": "command-aliases",
            "positional_arguments": [],
            "rargs_version": "0.0.0",
            "shebang": "#!/usr/bin/env bash",
            "version": "0.0.1"
        })
    );
}
