use std::collections::HashMap;

use color_eyre::eyre::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::value::{Map, Value};
use tera::{Context, Tera};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Merges two Map<String, Value> objects. The second object takes precedence over the first.
pub fn merge_maps(
    first: &mut Map<String, Value>,
    second: &Map<String, Value>,
) -> Map<String, Value> {
    for (key, second_value) in second {
        if let Some(first_value) = first.get_mut(key) {
            if let (Some(first_value), Some(second_value)) =
                (first_value.as_object_mut(), second_value.as_object())
            {
                merge_maps(first_value, second_value);
            } else {
                *first_value = second_value.clone();
            }
        } else {
            first.insert(key.clone(), second_value.clone());
        }
    }

    first.clone()
}

/// Tera filter that uses the merge_json_objects function to merge two values that are JSON objects.
pub fn merge_json_objects_filter(
    value: &Value,
    args: &HashMap<String, Value>,
) -> tera::Result<Value> {
    let with = args.get("with").ok_or_else(|| {
        tera::Error::msg("The filter `merge_json_objects` requires an argument named `with`")
    })?;
    let with = with.as_object().ok_or_else(|| {
        tera::Error::msg(format!(
            "The filter `merge_json_objects` received an invalid argument: {:?}",
            args
        ))
    })?;
    let value = value.as_object().ok_or_else(|| {
        tera::Error::msg(format!(
            "The filter `merge_json_objects` received an invalid value: {:?}",
            value
        ))
    })?;

    let first = merge_maps(&mut value.clone(), with);
    Ok(serde_json::to_value(first).unwrap())
}

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

        tera.register_filter("merge", merge_json_objects_filter);

        tera
    };
}

/// Render the main.sh template
pub fn render<T: Serialize>(context: &T) -> Result<String> {
    let output = TEMPLATES.render("main.tera", &Context::from_serialize(context)?)?;
    Ok(output)
}
