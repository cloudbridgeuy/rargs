use std::collections::HashMap;

use color_eyre::eyre::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::value::{Map, Value};
use tera::{Context, Tera};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn pairs_to_dot_columns(pairs: Vec<(&str, &str)>, indent: usize) -> String {
    // Get the maximum length of the name and description fields.
    let max = pairs
        .iter()
        .map(|(first, _)| first.len() + indent + 1)
        .max()
        .unwrap();

    // Create a string builder to store the output.
    let mut output = String::new();

    // Iterate over the commands and add them to the string builder.
    for (first, second) in pairs {
        // Pad the name and description fields to the maximum length.
        let padded_first = format!("{:.<width$}", "  ".to_string() + first + " ", width = max);

        // Add the name and description to the string builder, separated by dots.
        output.push_str(&format!("{}.... {}\n", padded_first, second));
    }

    // Return the string builder as a string.
    output
}

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

/// Tera filter that takes a collection and returns a dot column representation for two of its keys.
pub fn object_value_to_dot_columns(
    value: &Value,
    args: &HashMap<String, Value>,
) -> tera::Result<Value> {
    let key = args.get("key").ok_or_else(|| {
        tera::Error::msg("The filter `merge_json_objects` requires an argument named `second`")
    })?;
    let indent = args.get("indent").ok_or_else(|| {
        tera::Error::msg("The filter `merge_json_objects` requires an argument named `second`")
    })?;
    let key = key.as_str().ok_or_else(|| {
        tera::Error::msg(format!(
            "The filter `merge_json_objects` received an invalid argument: {:?}",
            args
        ))
    })?;
    let indent = indent.as_u64().ok_or_else(|| {
        tera::Error::msg(format!(
            "The filter `merge_json_objects` received an invalid argument: {:?}",
            args
        ))
    })? as usize;
    // Convert the value to an object then iterate over the keys and map its values to a
    // `Vec<( &str,&str )>` where the first value is the first key and the second value is
    // the second key.
    let pairs: Vec<(&str, &str)> = value
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, object)| {
            if object.get("private").is_some() {
                !object.get("private").unwrap().as_bool().unwrap()
            } else {
                true
            }
        })
        .map(|(name, object)| {
            (
                name.as_str(),
                if object.get(key).is_some() {
                    object.get(key).unwrap().as_str().unwrap()
                } else {
                    ""
                },
            )
        })
        .collect();

    Ok(serde_json::to_value(pairs_to_dot_columns(pairs, indent)).unwrap())
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
        tera.register_filter("object_value_to_dot_columns", object_value_to_dot_columns);

        tera
    };
}

/// Render the main.tera template
pub fn render<T: Serialize>(context: &T) -> Result<String> {
    let output = TEMPLATES.render("main.tera", &Context::from_serialize(context)?)?;
    Ok(output)
}

/// Render the minimal.tera template
pub fn minimal<T: Serialize>(context: &T) -> Result<String> {
    let output = TEMPLATES.render("minimal.tera", &Context::from_serialize(context)?)?;
    Ok(output)
}
