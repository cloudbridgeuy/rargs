use std::process::Command;
use test_log::test;

use utils::{build_script, test_script};

#[test]
fn test_scripts() {
    let scripts = vec![(
        "minimal.sh",
        vec![
            ("stderr", ""),
            ("stdout", "--help"),
            ("stderr", "-f"),
            ("stdout", "source -f"),
            ("stdout", "source target -f"),
        ],
    )];

    for (script, commands) in scripts {
        build_script!(script);

        for (stream, options) in commands {
            test_script!(script, stream, options);
        }
    }
}
