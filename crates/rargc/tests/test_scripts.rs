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
            },
        ),
        (
            "minus-v.sh",
            Commands {
                stdout: vec!["--help", "-h", "--version", "-v"],
                stderr: vec![],
            },
        ),
        (
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
