use std::collections::HashMap;

use serde::Serialize;

use crate::parser;

pub const EXTRA_ARGS: &str = "extra_args";

/// Stores the names of all the parameters.
pub type Names = (
    HashMap<String, parser::Position>,
    HashMap<char, parser::Position>,
);

/// A struct representin a param tag E.g. `@arg`, `@option`, `@flag`, `@env`
#[derive(Serialize, Debug, Clone, Default)]
pub struct Data {
    pub name: String,
    pub choices: std::option::Option<Vec<String>>,
    pub multiple: bool,
    pub required: bool,
    pub private: bool,
    pub default: std::option::Option<String>,
}

impl Data {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// A parameter trait for the `@option`, `@flag`, and `@arg` tags.
pub trait Param {
    fn name(&self) -> &str;
    fn tag_name(&self) -> &str;
}

/// Represents a `@flag` param. E.g. `@flag -h --help "Prints help information"`
#[derive(Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Flag {
    pub name: String,
    pub description: String,
    pub short: std::option::Option<char>,
    pub default: std::option::Option<String>,
    pub multiple: bool,
}

impl Flag {
    pub fn new(data: Data, description: &str, short: std::option::Option<char>) -> Self {
        Self {
            name: data.name,
            description: description.to_string(),
            short,
            default: data.default,
            multiple: data.multiple,
        }
    }
}

impl Param for Flag {
    fn name(&self) -> &str {
        &self.name
    }

    fn tag_name(&self) -> &str {
        "@flag"
    }
}

/// Represents an `@env` param. E.g. `@env FOO! description` or `@env FOO!`.
#[derive(Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Env {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub option: std::option::Option<String>,
}

impl Env {
    pub fn new(
        name: &str,
        description: std::option::Option<String>,
        required: bool,
        option: std::option::Option<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.unwrap_or_default(),
            required,
            option,
        }
    }
}

/// Represents a `@dep` param. E.g. `@dep foo`, `@dep foo,bar,bax`, `@dep foo Optional
/// message`, or `@dep foo,bar,bax Optional message`.
#[derive(Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Dep {
    pub list: Vec<String>,
    pub message: std::option::Option<String>,
    pub alias: std::option::Option<String>,
}

impl Dep {
    pub fn new(
        list: Vec<String>,
        message: std::option::Option<String>,
        alias: std::option::Option<String>,
    ) -> Self {
        Self {
            list,
            message,
            alias,
        }
    }
}

/// Represents an `@example` param. E.g. `@example Prints hello world $ greet`
#[derive(Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Example {
    pub description: String,
    pub command: String,
}

impl Example {
    pub fn new(description: &str, command: &str) -> Self {
        Self {
            description: description.to_string(),
            command: command.to_string(),
        }
    }
}

/// Represents an `@any` param. E.g. `@any! <VALUE_NOTATION> Any value requiring at least one."
#[derive(Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Any {
    pub description: std::option::Option<String>,
    pub value_notation: std::option::Option<String>,
    pub required: bool,
}

impl Any {
    pub fn new(
        description: std::option::Option<String>,
        value_notation: std::option::Option<String>,
        required: bool,
    ) -> Self {
        Self {
            description,
            value_notation,
            required,
        }
    }
}

/// Represents an `@arg` param. E.g. `@arg name Your name`.
#[derive(Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct PositionalArgument {
    pub name: String,
    pub description: String,
    pub choices: std::option::Option<Vec<String>>,
    pub required: bool,
    pub multiple: bool,
    pub default: std::option::Option<String>,
    pub value_notation: std::option::Option<String>,
}

impl PositionalArgument {
    pub fn new(data: Data, description: &str, value_notation: std::option::Option<String>) -> Self {
        Self {
            name: data.name,
            description: description.to_string(),
            choices: data.choices,
            required: data.required,
            multiple: data.multiple,
            default: data.default,
            value_notation,
        }
    }
}

/// Represents an `@option` param. E.g. `@option -n --name <name> "Your name"`
#[derive(Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Option {
    pub name: String,
    pub description: String,
    pub short: std::option::Option<char>,
    pub multiple: bool,
    pub required: bool,
    pub default: std::option::Option<String>,
    pub choices: std::option::Option<Vec<String>>,
    pub value_notation: std::option::Option<String>,
}

impl Option {
    pub fn new(
        data: Data,
        description: &str,
        short: std::option::Option<char>,
        value_notation: std::option::Option<String>,
    ) -> Self {
        Self {
            name: data.name,
            description: description.to_string(),
            choices: data.choices,
            multiple: data.multiple,
            required: data.required,
            default: data.default,
            short,
            value_notation,
        }
    }
}
