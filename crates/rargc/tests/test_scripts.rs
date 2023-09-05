use std::process::Command;
use test_log::test;

use utils::{build_script, test_script};

struct Commands {
    stdout: Vec<&'static str>,
    stderr: Vec<&'static str>,
}

#[test]
fn test_scripts() {
    let scripts = vec![
        (
            "minimal.sh",
            Commands {
                stdout: vec![
                    "--help",
                    "-h",
                    "--version",
                    "-v",
                    "source",
                    "source target",
                    "source target -f",
                    "source -f target",
                    "--fail",
                ],
                stderr: vec!["", "-f"],
            }
        ), (
            "minus-v.sh",
            Commands {
                stdout: vec!["--help", "-h", "--version", "-v"],
                stderr: vec![],
            },
        ), (
            "commands.sh",
            Commands {
                stdout: vec![
                    "--help",
                    "-h",
                    "--version",
                    "-v",
                    "download --help",
                    "download -h",
                    "download example.com",
                    "download example.com ./output",
                    "download example.com -f",
                    "download example.com ./output -f",
                    "download example.com -f ./output",
                    "d --help",
                    "d -h",
                    "d example.com",
                    "d example.com ./output",
                    "d example.com -f",
                    "d example.com ./output -f",
                    "d example.com -f ./output",
                    "upload --help",
                    "upload -h",
                    "upload source --user Foo",
                    "upload source --password Bar",
                    "upload source --password Foo --password Bar",
                    "u source --user Foo",
                    "u source --password Bar",
                    "u source --password Foo --password Bar",
                    ],
                    stderr: vec![
                        "",
                        "download",
                        "d",
                        "upload --user Foo",
                        "upload --password Bar",
                        "upload --password Foo --password Bar",
                        "u --help",
                        "u -h",
                        "u --user Foo",
                        "u --password Bar",
                        "u --user Foo --password Bar",
                    ],
            },
        ), (
            "commands-default.sh",
            Commands {
                stdout: vec![
                    "--help",
                    "source",
                    "source -f",
                    "upload --help",
                    "upload -h",
                    "upload source",
                    "upload source -f",
                    "download source",
                ],
                stderr: vec!["", "-f", "upload", "upload -f"],
            },
        ), (
            "commands-default-force.sh",
            Commands {
                stdout: vec![
                    "--help",
                    "source",
                    "upload --help",
                    "upload -h",
                    "upload source",
                    "download source",
                ],
                stderr: vec!["", "upload", "upload -f"],
            },
        ), (
            "commands-nested.sh",
            Commands {
                stdout: vec!["--help", "dir -h", "file -h", "dir list -h"],
                stderr: vec!["", "dir", "file", "dir list"],
            },
        ), (
            "commands-aliases.sh",
            Commands {
                stdout: vec![
                    "download --help",
                    "d --help",
                    "down --help",
                    "upload --help",
                    "u --help",
                    "push --help",
                    "push example.com",
                ],
                stderr: vec!["", "download", "upload", "d", "u", "down", "push"],
            },
        ), (
            "dependencies.sh",
            Commands {
                stdout: vec![],
                stderr: vec!["", "download", "upload"],
            },
        ), (
            "catch-all.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "other",
                    "required -h",
                    "multiple -h",
                    "no-multiple -h",
                    "multiple something",
                    "no-multiple something",
                    "multiple something with --additional args",
                    "no-multiple something with --additional args",
                    "multiple something with --additional args and --debug",
                    "no-multiple something with --additional args and --debug",
                    "multiple something with --additional args and --debug -- with something else",
                    "no-multiple something with --additional args and --debug -- with something else",
                    "other something with --additional args and --debug -- with something else",
                ],
                stderr: vec!["", "required"],
            },
        ), (
            "catch-all-global.sh",
            Commands {
                stdout: vec![
                    "",
                    "other",
                    "something, something, the dark side",
                    "-h",
                    "multiple -h",
                    "no-multiple -h",
                    "multiple something",
                    "no-multiple something",
                    "multiple something with --additional args",
                    "no-multiple something with --additional args",
                    "multiple something with --additional args and --debug",
                    "no-multiple something with --additional args and --debug",
                    "multiple something with --additional args and --debug -- with something else",
                    "no-multiple something with --additional args and --debug -- with something else",
                    "other something with --additional args and --debug -- with something else",
                ],
                stderr: vec![
                    "multiple",
                    "no-multiple",
                ],
            },
        ),
    ];

    for (script, commands) in scripts {
        build_script!(script);

        for options in commands.stderr {
            test_script!(script, "stderr", options);
        }

        for options in commands.stdout {
            test_script!(script, "stdout", options);
        }
    }
}
