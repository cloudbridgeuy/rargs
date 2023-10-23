use std::collections::HashMap;

use color_eyre::eyre::{Result, WrapErr};
use serde::Serialize;
use thiserror::Error;

use crate::param;
use crate::parser;

#[derive(Serialize, Default, Debug, Clone)]
pub struct Command {
    pub aliases: Option<Vec<String>>,
    pub description: Option<String>,
    pub examples: Option<Vec<param::Example>>,
    pub flags: HashMap<String, param::Flag>,
    pub help: Option<String>,
    pub comments: Option<Vec<String>>,
    pub lines: Option<Vec<String>>,
    pub name: Option<String>,
    pub options: HashMap<String, param::Option>,
    pub positional_arguments: Vec<param::PositionalArgument>,
    pub rules: Option<Vec<String>>,
    pub subcommand: Option<String>,
    pub dep: Option<Vec<param::Dep>>,
    pub envs: HashMap<String, param::Env>,
    pub any: Option<param::Any>,
    pub private: bool,
}

impl Command {
    pub fn new() -> Self {
        Self {
            lines: Vec::new().into(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Default, Debug)]
pub struct Script {
    pub author: Option<Vec<String>>,
    pub commands: HashMap<String, Command>,
    pub default: Option<String>,
    pub description: Option<String>,
    pub examples: Option<Vec<param::Example>>,
    pub flags: HashMap<String, param::Flag>,
    pub help: Option<String>,
    pub comments: Option<Vec<String>>,
    pub lines: Option<Vec<String>>,
    pub name: Option<String>,
    pub options: HashMap<String, param::Option>,
    pub positional_arguments: Vec<param::PositionalArgument>,
    pub rargs_version: String,
    pub rules: Option<Vec<String>>,
    pub shebang: String,
    pub version: Option<String>,
    pub dep: Option<Vec<param::Dep>>,
    pub root: Option<Vec<String>>,
    pub envs: HashMap<String, param::Env>,
    pub any: Option<param::Any>,
}

impl Script {
    pub fn new() -> Self {
        Self {
            rargs_version: env!("CARGO_PKG_VERSION").to_string(),
            shebang: "#!/usr/bin/env bash".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("Unknown error")]
    Unknown,
    #[error("No command in scope")]
    NoCommandInScope(String),
    #[error("Duplicate argument name")]
    DuplicateArgument(String),
    #[error("Can't declare an argument after a declaring a multiple argument")]
    NoArgumentAfterMultiple(String),
    #[error("Can't define an alias at the root scope")]
    NoAliasAtRootScope(String),
    #[error("Can't define a subcommand at the root scope")]
    NoSubcommandAtRootScope(String),
    #[error("Duplicate any")]
    DuplicateAny(String),
}

static RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^\}$").unwrap());
const ROOT: &str = "root";

impl Script {
    pub fn from_source(source: &str) -> Result<Self> {
        let mut is_root_scope = true;
        let mut script = Script::new();
        let mut maybe_command: Option<Command> = None;
        let mut last_command: Option<String> = None;

        let events = parser::parse_source(source)?;

        let mut err: Option<ScriptError> = None;
        for event in events {
            match event.data.clone() {
                parser::Data::Name(value) => {
                    script.name = Some(value);
                }
                parser::Data::Description(value) => {
                    if is_root_scope {
                        script.description = Some(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.description = Some(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope when parsing a @description param in line {}. Did you forget the @cmd directive?",
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Author(value) => {
                    script.author = Some(value);
                }
                parser::Data::Version(value) => {
                    script.version = Some(value);
                }
                parser::Data::Private => {
                    if let Some(command) = maybe_command.as_mut() {
                        command.private = true;
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope when parsing a @private param in line {}. Did you forget the @cmd directive?",
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Help(value) => {
                    if is_root_scope {
                        script.help = Some(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.help = Some(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope when parsing a @help param in line {}. Did you forget the @cmd directive?",
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Func(value) => {
                    if let Some(command) = maybe_command.as_mut() {
                        command.name = Some(value.clone());
                        script
                            .commands
                            .entry(value.clone())
                            .or_insert(command.clone());
                        last_command = Some(value);
                    } else if value == ROOT {
                        is_root_scope = false;
                        script.root = Some(Vec::new());
                        last_command = Some(ROOT.to_string());
                    } else {
                        script
                            .lines
                            .get_or_insert_with(Vec::new)
                            .push(format!("{}() {{", value));
                    }
                    maybe_command = None;
                }
                parser::Data::Unknown(value) => {
                    log::warn!("unknown: {}", value);
                }
                parser::Data::Flag(value) => {
                    if is_root_scope {
                        script.flags.entry(value.name.clone()).or_insert(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.flags.entry(value.name.clone()).or_insert(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing flag --{} in line {}. Did you forget the @cmd directive?",
                            value.name,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Cmd(value) => {
                    is_root_scope = false;
                    let mut command = Command::new();
                    command.description = Some(value);
                    maybe_command = Some(command);
                }
                parser::Data::Default(value) => {
                    script.default = Some(value);
                }
                parser::Data::Env(value) => {
                    if is_root_scope {
                        script.envs.entry(value.name.clone()).or_insert(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.envs.entry(value.name.clone()).or_insert(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing env --{} in line {}. Did you forget the @cmd directive?",
                            value.name,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Option(value) => {
                    if is_root_scope {
                        script.options.entry(value.name.clone()).or_insert(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.options.entry(value.name.clone()).or_insert(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing option --{} in line {}. Did you forget the @cmd directive?",
                            value.name,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Any(value) => {
                    if is_root_scope {
                        if script.any.is_some() {
                            err = Some(ScriptError::DuplicateAny(format!(
                                "Duplicate @any in line {}.",
                                event.position
                            )));
                            break;
                        } else {
                            script.any = Some(value);
                        };
                    } else if let Some(command) = maybe_command.as_mut() {
                        if command.any.is_some() {
                            err = Some(ScriptError::DuplicateAny(format!(
                                "Duplicate @any in line {}.",
                                event.position
                            )));
                            break;
                        } else {
                            command.any = Some(value);
                        };
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing @any in line {}. Did you forget the @cmd directive?",
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::PositionalArgument(value) => {
                    if is_root_scope {
                        // Check if the arg is already declared.
                        if script
                            .positional_arguments
                            .iter()
                            .any(|v| v.name == value.name)
                        {
                            err = Some(ScriptError::DuplicateArgument(format!(
                                "Duplicate arg {} for in line {}.",
                                value.name, event.position,
                            )));
                            break;
                        }

                        // Check if there is a previous positional argument.
                        if let Some(arg) = script.positional_arguments.last_mut() {
                            if arg.multiple {
                                err = Some(ScriptError::NoArgumentAfterMultiple(format!(
                                    "Arg {} in line {} is declared after a multiple arg.",
                                    value.name, event.position
                                )));
                                break;
                            }
                            arg.required = true;
                        }

                        script.positional_arguments.push(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        // Check if the arg is already declared.
                        if command
                            .positional_arguments
                            .iter()
                            .any(|v| v.name == value.name)
                        {
                            err = Some(ScriptError::DuplicateArgument(format!(
                                "Duplicate arg {} for in line {}.",
                                value.name, event.position
                            )));
                            break;
                        }

                        // Check if there is a previous multiple positional argument.
                        if let Some(arg) = command.positional_arguments.last_mut() {
                            if arg.multiple {
                                err = Some(ScriptError::NoArgumentAfterMultiple(format!(
                                    "Arg {} in line {} is declared after a multiple arg.",
                                    value.name, event.position
                                )));
                                break;
                            }
                            arg.required = true;
                        }

                        command.positional_arguments.push(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing arg {} in line {}. Did you forget the @cmd directive?",
                            value.name,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::SheBang(value) => {
                    script.shebang = value;
                }
                parser::Data::Rule(value) => {
                    if is_root_scope {
                        script.rules.get_or_insert_with(Vec::new).push(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.rules.get_or_insert_with(Vec::new).push(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing rule {} in line {}. Did you forget the @cmd directive?",
                            value,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Alias(value) => {
                    if is_root_scope {
                        err = Some(ScriptError::NoAliasAtRootScope(format!(
                            "Aliases are not supported at the root scope. Found declaration for alias {} in line {}.",
                            value,
                            event.position
                        )));
                        break;
                    } else if let Some(command) = maybe_command.as_mut() {
                        // Check if the value has spaces, and if it does, split it into multiple aliases.
                        for alias in value.split(' ') {
                            command
                                .aliases
                                .get_or_insert_with(Vec::new)
                                .push(alias.to_string());
                        }
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing alias {} in line {}. Did you forget the @cmd directive?",
                            value,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Subcommand(value) => {
                    if is_root_scope {
                        err = Some(ScriptError::NoSubcommandAtRootScope(format!(
                            "Subcommands are not supported at the root scope. Found declaration for subcommand {} in line {}.",
                            value,
                            event.position
                        )));
                        break;
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.subcommand = Some(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing subcommand {} in line {}. Did you forget the @cmd directive?",
                            value,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Example(value) => {
                    if is_root_scope {
                        script.examples.get_or_insert_with(Vec::new).push(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.examples.get_or_insert_with(Vec::new).push(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing example {:?} in line {}. Did you forget the @cmd directive?",
                            value,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Dep(value) => {
                    if is_root_scope {
                        script.dep.get_or_insert_with(Vec::new).push(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.dep.get_or_insert_with(Vec::new).push(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing dep {:?} in line {}. Did you forget the @cmd directive?",
                            value,
                            event.position
                        )));
                        break;
                    }
                }
                parser::Data::Comment(value) => {
                    if is_root_scope {
                        script.comments.get_or_insert_with(Vec::new).push(value);
                    } else if let Some(command) = &mut maybe_command {
                        command.comments.get_or_insert_with(Vec::new).push(value);
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No command in scope in when parsing comment `{}` in line {}. Did you forget the @cmd directive?",
                            value,
                            event.position
                        )));
                        break;
                    }
                }

                parser::Data::Line(value) => {
                    if is_root_scope {
                        script.lines.get_or_insert_with(Vec::new).push(value);
                    } else if let Some(last_command) = &last_command {
                        if RE.is_match(&value) {
                            is_root_scope = true;
                        } else if last_command == ROOT {
                            if let Some(root) = script.root.as_mut() {
                                root.push(value);
                            } else {
                                err = Some(ScriptError::NoCommandInScope(format!(
                                    "No root command in scope in when parsing `{}` in line {}. Did you forget the @cmd directive?",
                                    value,
                                    event.position
                                )));
                                break;
                            }
                        } else {
                            script
                                .commands
                                .get_mut(last_command)
                                .unwrap()
                                .lines
                                .get_or_insert_with(Vec::new)
                                .push(value);
                        }
                    } else {
                        err = Some(ScriptError::NoCommandInScope(format!(
                            "No root command in scope in when parsing `{}` in line {}. Did you forget the @cmd directive?",
                            value,
                            event.position
                        )));
                        break;
                    }
                }
            }
        }

        if let Some(err) = err {
            log::error!("Error while parsing script: {:#?}", &err);
            return Err(err).wrap_err("Error while parsing script.");
        }

        log::debug!("script: {:#?}", &script);

        Ok(script)
    }
}
