use std::process::Command;
use test_log::test;

use utils::{build_script, test_script};

struct Commands {
    stdout: Vec<&'static str>,
    stderr: Vec<&'static str>,
}

#[test]
fn test_minimal_sh_script() {
    let script = "minimal.sh";
    let commands = Commands {
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
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_minus_v_sh_script() {
    let script = "minus-v.sh";
    let commands = Commands {
        stdout: vec!["--help", "-h", "--version", "-v"],
        stderr: vec![],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_commands_sh_script() {
    let script = "commands.sh";
    let commands = Commands {
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
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_commands_default() {
    let script = "commands-default.sh";
    let commands = Commands {
        stdout: vec![
            "--help",
            "-h",
            "--version",
            "-v",
            "source",
            "source -f",
            "upload --help",
            "upload -h",
            "upload source",
            "upload source -f",
            "download source",
        ],
        stderr: vec!["", "-f", "upload", "upload -f"],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_commands_default_force() {
    let script = "commands-default-force.sh";
    let commands = Commands {
        stdout: vec![
            "--help",
            "-h",
            "--version",
            "-v",
            "source",
            "source -f",
            "upload --help",
            "upload -h",
            "upload source",
            "upload source -f",
            "download source",
        ],
        stderr: vec!["", "-f", "upload", "upload -f"],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_commands_nested() {
    let script = "commands-nested.sh";
    let commands = Commands {
        stdout: vec![
            "--help",
            "-h",
            "--version",
            "-v",
            "dir --help",
            "dir -h",
            "dir list --help",
            "dir list -h",
            "file --help",
            "file -h",
        ],
        stderr: vec!["", "dir", "file"],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_command_aliases() {
    let script = "commands-aliases.sh";
    let commands = Commands {
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
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_dependencies() {
    let scripts = "dependencies.sh";
    let commands = Commands {
        stdout: vec![],
        stderr: vec!["", "download", "upload"],
    };

    build_script!(scripts);

    for options in commands.stderr {
        test_script!(scripts, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(scripts, "stdout", options);
    }
}

#[test]
fn test_catch_all() {
    let script = "catch-all.sh";
    let commands = Commands {
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
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_catch_all_global() {
    let script = "catch-all-global.sh";
    let commands = Commands {
        stdout: vec![
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
        stderr: vec!["", "multiple", "no-multiple"],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_catch_all_stdin() {
    let script = "catch-all-stdin.sh";
    let commands = Commands {
        stdout: vec!["-h", "<<<foo"],
        stderr: vec!["--format toml"],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_extensible() {
    let script = "extensible.sh";
    let commands = Commands {
        stdout: vec![
            "-h",
            "example --debug -f",
            "download -h",
            "download SOURCE",
            "upload -h",
            "upload SOURCE",
        ],
        stderr: vec!["", "upload", "download"],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_extensible_delegate() {
    let script = "extensible-delegate.sh";
    let commands = Commands {
        stdout: vec![
            "-h",
            "log --pretty",
            "download -h",
            "download SOURCE",
            "upload -h",
            "upload SOURCE",
        ],
        stderr: vec!["", "upload", "download"],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_whitelist() {
    let script = "whitelist.sh";
    let commands = Commands {
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
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_repeatable_arg() {
    let script = "repeatable-arg.sh";
    let commands = Commands {
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
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_repeatable_flag() {
    let script = "repeatable-flag.sh";
    let commands = Commands {
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
        stderr: vec![],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_private() {
    let script = "private.sh";
    let commands = Commands {
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
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_stdin() {
    let script = "stdin.sh";
    let commands = Commands {
        stdout: vec!["-h", "arg --help", "option --help"],
        stderr: vec![],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_examples() {
    let script = "examples.sh";
    let commands = Commands {
        stdout: vec!["-h"],
        stderr: vec![],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_empty() {
    let script = "empty.sh";
    let commands = Commands {
        stdout: vec!["-v", "--version", "-help", "-h"],
        stderr: vec![""],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        println!("stdout: {}", options);
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_hello_world() {
    let script = "hello-world.sh";
    let commands = Commands {
        stdout: vec!["", "-v", "--version", "-help", "-h"],
        stderr: vec![],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_huggingface() {
    let script = "huggingface.sh";
    let commands = Commands {
        stdout: vec!["", "-v", "--version", "-help", "-h"],
        stderr: vec![],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_first_option_help() {
    let script = "first-option-help.sh";
    let commands = Commands {
        stdout: vec![
            "",
            "-h",
            "--help",
            "download -h",
            "download --help",
            "upload -h",
            "upload --help",
        ],
        stderr: vec![
            "-v -h",
            "-v --help",
            "download source -h",
            "download source --help",
            "upload source -h",
            "upload source --help",
        ],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        println!("stdout: {}", options);
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_no_first_option_help_global() {
    let script = "no-first-option-help-global.sh";
    let commands = Commands {
        stdout: vec![
            "",
            "-h",
            "--help",
            "download -h",
            "download --help",
            "upload -h",
            "upload --help",
            "-v -h",
            "-v --help",
            "download source -h",
            "download source --help",
            "upload source -h",
            "upload source --help",
        ],
        stderr: vec![],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_no_first_option_help_local() {
    let script = "no-first-option-help-local.sh";
    let commands = Commands {
        stdout: vec![
            "",
            "-h",
            "--help",
            "download -h",
            "download --help",
            "upload -h",
            "upload --help",
            "download source -h",
            "download source --help",
        ],
        stderr: vec![
            "-v -h",
            "-v --help",
            "upload source -h",
            "upload source --help",
        ],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_minimal_with_comments() {
    let script = "minimal-with-comments.sh";
    let commands = Commands {
        stdout: vec!["-h", "--help"],
        stderr: vec![""],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_json() {
    let script = "json.sh";
    let commands = Commands {
        stdout: vec!["", "-h", "--help"],
        stderr: vec![],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}

#[test]
fn test_flags() {
    let script = "flags.sh";
    let commands = Commands {
        stdout: vec![
            "--falsy",
            "--no-truthy",
            "--truthy",
            "--shorty",
            "--no-shorty",
            "-no-s",
            "-h",
            "--help",
            "",
        ],
        stderr: vec![
            "--falsy",
            "--no-truthy",
            "--truthy",
            "--shorty",
            "--no-shorty",
            "-no-s",
            "",
        ],
    };

    build_script!(script);

    for options in commands.stderr {
        test_script!(script, "stderr", options);
    }

    for options in commands.stdout {
        test_script!(script, "stdout", options);
    }
}
