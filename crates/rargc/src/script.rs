use color_eyre::eyre::Result;

use crate::parser;

#[derive(Default, Debug)]
pub struct Script {
    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<Vec<String>>,
    pub version: Option<String>,
    pub help: Option<String>,
}

impl Script {
    pub fn from_source(source: &str) -> Result<Self> {
        let mut script = Script::default();

        let events = parser::parse_source(source)?;

        for event in events {
            match event.data {
                parser::Data::Name(value) => {
                    script.name = Some(value);
                }
                parser::Data::Description(value) => {
                    script.description = Some(value);
                }
                parser::Data::Author(value) => {
                    script.author = Some(value);
                }
                parser::Data::Version(value) => {
                    script.version = Some(value);
                }
                parser::Data::Help(value) => {
                    script.help = Some(value);
                }
                parser::Data::Func(value) => {
                    println!("func: {}", value);
                }
                parser::Data::Unknown(value) => {
                    println!("unknown: {}", value);
                }
            }
        }

        Ok(script)
    }
}
