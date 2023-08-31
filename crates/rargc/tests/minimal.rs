use std::process::Command;
use test_log::test;

use utils::{build_script, test_script};

#[test]
fn test_minimal() {
    let script_name = "minimal.sh".to_string();

    let commands: Vec<(&str, &str)> = vec![
        ("stderr", ""),
        ("stdout", "--help"),
        ("stderr", "-f"),
        ("stdout", "source -f"),
        ("stdout", "source target -f"),
    ];

    build_script!(script_name);

    for (stream, options) in commands {
        test_script!(script_name, stream, options);
    }
}
