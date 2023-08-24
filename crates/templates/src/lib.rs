use color_eyre::eyre::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.tera") {
            Ok(t) => t,
            Err(e) => {
                log::error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        for entry in TEMPLATES_DIR.files() {
            match tera.add_raw_template(
                entry.path().to_str().unwrap(),
                entry.contents_utf8().unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => {
                    log::error!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            }
        }

        tera
    };
}

/// Render the main.sh template
pub fn render<T: Serialize>(context: &T) -> Result<String> {
    let output = TEMPLATES.render("main.tera", &Context::from_serialize(context)?)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_sh() {
        let objects = vec![
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "foo": {
                        "short": "f",
                        "name": "foo",
                        "summary": "Test foo option"
                    }
                }
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "required": {
                        "short": "r",
                        "name": "required",
                        "summary": "Test required option",
                        "required": "true"
                    }
                }
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "multiple": {
                        "short": "m",
                        "name": "multiple",
                        "summary": "Test multiple option",
                        "multiple": "true"
                    }
                }
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "default": {
                        "short": "d",
                        "name": "default",
                        "summary": "Test default option",
                        "default": "foo"
                    }
                }
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "choices": {
                        "short": "c",
                        "name": "choices",
                        "summary": "Test option with choices",
                        "choices": ["foo", "bar", "baz"]
                    }
                }
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "required": {
                        "short": "rm",
                        "name": "required-multiple",
                        "summary": "Test required and multiple option",
                        "required": "true",
                        "multiple": "true"
                    }
                }
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "default": {
                        "short": "dm",
                        "name": "default-multiple",
                        "summary": "Test default and multiple option",
                        "default": "foo",
                        "multiple": "true"
                    }
                }
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "choices": {
                        "short": "cm",
                        "name": "choices-multiple",
                        "summary": "Test option with choices and multiple",
                        "choices": ["foo", "bar", "baz"],
                        "multiple": "true"
                    }
                }
            })),
            Context::from_serialize(serde_json::json!({
                "meta": {
                    "name": "usage",
                    "description": "Test simple usage"
                },
                "options": {
                    "all": {
                        "short": "a",
                        "name": "all",
                        "summary": "Test option with all attributes",
                        "choices": ["foo", "bar", "baz"],
                        "multiple": "true",
                        "required": "true",
                        "default": "foo"
                    }
                }
            })),
        ];

        for object in objects {
            let output =
                match TEMPLATES.render("usage.tera", &object.expect("Can't create JSON object")) {
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
}
