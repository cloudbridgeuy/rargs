use crate::utils::hyphens_to_underscores;

pub const VARIABLE_PREFIX: &str = "rargc";

#[derive(Debug, PartialEq, Eq)]
pub enum RargcValue {
    Single(String, String),
    Multiple(String, Vec<String>),
    PositionalSingle(String, String),
    PositionalMultiple(String, Vec<String>),
    FnName(String),
}

impl RargcValue {
    pub fn to_shell(values: Vec<Self>) -> String {
        let mut variables = vec![];
        let mut positional_args = vec![];

        for value in values {
            match value {
                RargcValue::Single(name, value) => {
                    variables.push(format!(
                        "{}_{}={}",
                        VARIABLE_PREFIX,
                        hyphens_to_underscores(&name),
                        value
                    ));
                }
                RargcValue::Multiple(name, values) => {
                    variables.push(format!(
                        "{}_{}=( {} )",
                        VARIABLE_PREFIX,
                        hyphens_to_underscores(&name),
                        values.join(" ")
                    ));
                }
                RargcValue::PositionalSingle(name, value) => {
                    positional_args.push(format!(
                        "{}_{}={}",
                        VARIABLE_PREFIX,
                        hyphens_to_underscores(&name),
                        value
                    ));
                }
                RargcValue::PositionalMultiple(name, values) => {
                    positional_args.push(format!(
                        "{}_{}=( {} )",
                        VARIABLE_PREFIX,
                        hyphens_to_underscores(&name),
                        values.join(" ")
                    ));
                }
                RargcValue::FnName(name) => {
                    if positional_args.is_empty() {
                        variables.push(name.to_string());
                    } else {
                        variables.push(format!("{} {}", name, positional_args.join(" ")));
                    }
                }
            }
        }

        variables.join("\n")
    }
}
