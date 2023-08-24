use tera::Context;
use test_log::test;

use templates::TEMPLATES;

#[test]
fn test_render() {
    let objects = vec![
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
        })),
        Context::from_serialize(serde_json::json!({
            "name": "foo".to_string(),
            "command": {
                "short": "f".to_string()
            }
        })),
    ];

    for object in objects {
        let output = match TEMPLATES.render(
            "command_argument.tera",
            &object.expect("Can't create JSON object"),
        ) {
            Ok(o) => o,
            Err(e) => {
                log::error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        println!("output: {:#?}", output);

        insta::assert_snapshot!(output)
    }
}
