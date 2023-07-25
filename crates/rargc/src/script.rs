use std::collections::HashMap;

use color_eyre::eyre::{self, Result};

use crate::param;
use crate::parser;

#[derive(Default, Debug, Clone)]
pub struct Meta {
    pub name: Option<String>,
    pub description: Option<String>,
    pub help: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct Command {
    pub meta: Meta,
    pub flags: HashMap<String, param::Flag>,
    pub options: HashMap<String, param::Option>,
    pub positional_arguments: Vec<param::PositionalArgument>,
}

impl Command {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct Script {
    pub shebang: String,
    pub meta: Meta,
    pub flags: HashMap<String, param::Flag>,
    pub options: HashMap<String, param::Option>,
    pub author: Option<Vec<String>>,
    pub version: Option<String>,
    pub default: Option<String>,
    pub commands: HashMap<String, Command>,
}

impl Script {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta,
            shebang: "#!/usr/bin/env bash".to_string(),
            ..Default::default()
        }
    }
}

impl Script {
    pub fn from_source(source: &str) -> Result<Self> {
        let mut is_root_scope = true;
        let meta = Meta::default();
        let mut script = Script::new(meta);
        let mut maybe_command: Option<Command> = None;

        let events = parser::parse_source(source)?;

        for event in events {
            match event.data {
                parser::Data::Name(value) => {
                    script.meta.name = Some(value);
                }
                parser::Data::Description(value) => {
                    if is_root_scope {
                        script.meta.description = Some(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.meta.description = Some(value);
                    } else {
                        eyre::bail!(
                            "No command in scope when parsing a @description param in line {}. Did you forget the @cmd directive?",
                            event.position
                        );
                    }
                }
                parser::Data::Author(value) => {
                    script.author = Some(value);
                }
                parser::Data::Version(value) => {
                    script.version = Some(value);
                }
                parser::Data::Help(value) => {
                    if is_root_scope {
                        script.meta.help = Some(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.meta.help = Some(value);
                    } else {
                        eyre::bail!(
                            "No command in scope when parsing a @help param in line {}. Did you forget the @cmd directive?",
                            event.position
                        );
                    }
                }
                parser::Data::Func(value) => {
                    is_root_scope = false;

                    if let Some(command) = maybe_command.as_mut() {
                        command.meta.name = Some(value.clone());
                        script.commands.entry(value).or_insert(command.clone());
                    }
                }
                parser::Data::Unknown(value) => {
                    // TODO: Change this to a debug! tracing command.
                    println!("unknown: {}", value);
                }
                parser::Data::Flag(value) => {
                    if is_root_scope {
                        script.flags.entry(value.name.clone()).or_insert(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.flags.entry(value.name.clone()).or_insert(value);
                    } else {
                        eyre::bail!(
                            "No command in scope in when parsing flag --{} in line {}. Did you forget the @cmd directive?",
                            value.name,
                            event.position
                        );
                    }
                }
                parser::Data::Cmd(value) => {
                    is_root_scope = false;

                    let mut meta = Meta::default();

                    if !value.is_empty() {
                        meta.description = Some(value);
                    }

                    maybe_command = Some(Command::new(meta));
                }
                parser::Data::Default(value) => {
                    script.default = Some(value);
                }
                parser::Data::Option(value) => {
                    if is_root_scope {
                        script.options.entry(value.name.clone()).or_insert(value);
                    } else if let Some(command) = maybe_command.as_mut() {
                        command.options.entry(value.name.clone()).or_insert(value);
                    } else {
                        eyre::bail!(
                            "No command in scope in when parsing option --{} in line {}. Did you forget the @cmd directive?",
                            value.name,
                            event.position
                        );
                    }
                }
                parser::Data::PositionalArgument(value) => {
                    if is_root_scope {
                        eyre::bail!(
                            "Args are not supported at the root scope. Found declaration for arg {} in line {}.",
                            value.name,
                            event.position
                        )
                    } else if let Some(command) = maybe_command.as_mut() {
                        if command
                            .positional_arguments
                            .iter()
                            .any(|v| v.name == value.name)
                        {
                            eyre::bail!(
                                "Duplicate arg {} for in line {}.",
                                value.name,
                                event.position
                            );
                        }
                        command.positional_arguments.push(value);
                    } else {
                        eyre::bail!(
                            "No command in scope in when parsing arg {} in line {}. Did you forget the @cmd directive?",
                            value.name,
                            event.position
                        );
                    }
                }
                parser::Data::SheBang(value) => {
                    script.shebang = value;
                }
            }
        }

        Ok(script)
    }
}
