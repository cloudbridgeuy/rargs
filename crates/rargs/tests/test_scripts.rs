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
        ), (
            "catch-all-stdin.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "<<<foo",
                ],
                stderr: vec![
                    "--format toml"
                ],
            },
        ), (
            "extensible.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "example --debug -f",
                    "download -h",
                    "download SOURCE",
                    "upload -h",
                    "upload SOURCE",
                ],
                stderr: vec![
                    "",
                    "upload",
                    "download",
                ],
            },
        ), (
            "extensible-delegate.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "log --pretty",
                    "download -h",
                    "download SOURCE",
                    "upload -h",
                    "upload SOURCE",
                ],
                stderr: vec![
                    "",
                    "upload",
                    "download",
                ],
            },
        ), (
            "whitelist.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "eu --user admin",
                    "eu production --user admin --protocol ssh",
                ],
                stderr: vec![
                    "",
                    "america",
                    "america --user admin",
                    "eu --user hacker",
                    "eu --user hacker --protocol icmp",
                ],
            },
        ), (
            "repeatable-arg.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "file1",
                    "file file2 file3* --action downcase",
                    "file file2 file3 --action downcase --action upcase",
                    "formats -h",
                    "formats --action downcase",
                    "formats jpg --action downcase",
                    "formats jpg --action downcase gif",
                    "formats jpg png --action downcase gif",
                    "formats jpg --action upcase png --action downcase gif",
                ],
                stderr: vec![
                    "",
                    "file file2 file3 --action downcase --action --upcase",
                    "formats jpg tiff --action upcase png --action downcase gif",
                ],
            },
        ), (
            "repeatable-flag.sh",
            Commands {
                stdout: vec![
                    "",
                    "-h",
                    "-d 1 -d 2 -d3",
                    "-d 1 -d 2 -d 3",
                    "-d 1 -d 2 -d 3 -v",
                    "-d 1 -d 2 -d 3 -v -v",
                    "-d 1 -d 2 -d 3 -v -v -v",
                    "-d 1 -d 2 -d 3 -vvv",
                    "-d 1 -d 2 -d 3 -vv -v",
                ],
                stderr: vec![
                ],
            },
        ), (
            "private.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "connect --protocol ssh localhost",
                    "connect --protocol ftp localhost",
                    "connect --protocol ftp -v localhost",
                    "connect --protocol ssh -v localhost",
                    "connect-ssh --username foo localhost",
                    "connect-ftp --username foo localhost",
                ],
                stderr: vec![
                    "",
                    "connect --protocol http -v",
                    "connect-ftp -v --username foo localhost",
                    "connect-ssh --protocol ftp --username foo localhost",
                ],
            },
        ), (
            "stdin.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "arg --help",
                    "option --help",
                ],
                stderr: vec![
                ],
            },
        ), (
            "novelai.sh",
            Commands {
                stdout: vec![
                    "-h",
                    "generate-stream --help",
                ],
                stderr: vec![
                ],
            },
        ), (
            "examples.sh",
            Commands {
                stdout: vec![
                    "-h",
                ],
                stderr: vec![
                ],
            },
        ), (
            "empty.sh",
            Commands {
                stdout: vec![
                    "-v",
                    "--version",
                    "-help",
                    "-h",
                ],
                stderr: vec![
                    "",
                ],
            },
        ), (
            "hello-world.sh",
            Commands {
                stdout: vec![
                    "",
                    "-v",
                    "--version",
                    "-help",
                    "-h",
                ],
                stderr: vec![
                ],
            },
        ), (
            "huggingface.sh",
            Commands {
                stdout: vec![
                    "",
                    "-v",
                    "--version",
                    "-help",
                    "-h",
                ],
                stderr: vec![
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
