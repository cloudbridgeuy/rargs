use std::fs;

use anyhow::{anyhow, Result};

use crate::parser::{parse_line, Event};

pub struct Command {
    options: Options,
}

pub struct Options {
    pub script_root: String,
}

pub const EXTRA_ARGS: &str = "_args";

impl Command {
    /// Create a new command
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    /// Run the command
    pub fn run(&self) -> Result<()> {
        let source = fs::read_to_string(&self.options.script_root)
            .map_err(|e| anyhow::format_err!("Failed to read script root: {}", e))?;

        // let name = Path::new(&self.options.script_root)
        //     .file_name()
        //     .and_then(|v| v.to_str())
        //     .ok_or_else(|| anyhow!("Failed to get script name"))?;

        let events = self.parse(&source)?;

        println!("events: {:#?}", events);

        Ok(())
    }

    /// Tokenize the source
    fn parse(&self, source: &str) -> Result<Vec<Event>> {
        let mut result = vec![];

        for (index, line) in source.lines().enumerate() {
            let position = index + 1;

            match parse_line(line) {
                Ok((_, maybe_token)) => {
                    if let Some(maybe_data) = maybe_token {
                        if let Some(data) = maybe_data {
                            result.push(Event { data, position });
                        } else {
                            Err(anyhow!(
                                "syntax error on line \"{}\" at position: {}",
                                line,
                                position
                            ))?;
                        }
                    }
                }
                Err(e) => {
                    Err(anyhow!(
                        "fail to parse line \"{}\" at position \"{}\" with error: {}",
                        line,
                        position,
                        e
                    ))?;
                }
            }
        }

        Ok(result)
    }
}
