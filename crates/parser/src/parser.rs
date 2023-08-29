use color_eyre::eyre::{self, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use crate::param;

pub type Position = usize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub data: Data,
    pub position: Position,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum Data {
    SheBang(String),
    Author(Vec<String>),
    Alias(String),
    PositionalArgument(param::PositionalArgument),
    Cmd(String),
    Description(String),
    Example(param::Example),
    Flag(param::Flag),
    Func(String),
    Help(String),
    Name(String),
    Option(param::Option),
    Version(String),
    Default(String),
    Line(String),
    Rule(String),
    Subcommand(String),
    Unknown(String),
}

/// Parse a source string into a list of tokens.
pub fn parse_source(source: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];

    for (index, line) in source.lines().enumerate() {
        let position = index + 1;

        match parse_line(line) {
            Ok(Some(token)) => tokens.push(Token {
                data: token,
                position,
            }),
            Ok(None) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(tokens)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\}\s*$|\s*$)").unwrap();
}

/// Parse a line into a token.
pub fn parse_line(line: &str) -> Result<Option<Data>> {
    let maybe = nom::branch::alt((
        nom::combinator::map(nom::branch::alt((parse_tag, parse_fn, parse_shebang)), Some),
        nom::combinator::success(None),
    ))(line);

    match maybe {
        Ok((_rest_of_line, maybe_token)) => {
            if let Some(maybe_data) = maybe_token {
                if let Some(data) = maybe_data {
                    Ok(Some(data))
                } else {
                    Ok(Some(Data::Unknown(line.to_string())))
                }
            } else if !RE.is_match(_rest_of_line) {
                Ok(Some(Data::Line(_rest_of_line.to_string())))
            } else {
                Ok(None)
            }
        }
        Err(e) => Err(eyre::format_err!(
            "fail to parse line \"{}\" with error: {}",
            line,
            e
        )),
    }
}

/// Parses an input as if it was a shebang #! line.
fn parse_shebang(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::sequence::pair(
                nom::character::complete::char('#'),
                nom::character::complete::char('!'),
            ),
            parse_tail,
        ),
        |text| Some(Data::SheBang(format!("#!{}", text))),
    )(input)
}

/// Parses an input as if it was an example definition like `# @example Example description $
/// command --option`.
fn parse_example(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::sequence::pair(
                nom::bytes::complete::tag("example"),
                nom::character::complete::space1,
            ),
            nom::sequence::tuple((
                nom::bytes::complete::take_till(|c| c == '$'),
                nom::sequence::preceded(nom::character::complete::char('$'), parse_tail),
            )),
        ),
        |(description, command)| {
            Some(Data::Example(param::Example::new(
                description.trim(),
                command.trim(),
            )))
        },
    )(input)
}

/// Parses an input as if it was a bash comment with a tag such as
/// `# @name rest...`, `# @description rest...`, etc.
fn parse_tag(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::sequence::preceded(
        nom::sequence::tuple((
            nom::multi::many1(nom::character::complete::char('#')),
            nom::character::complete::space0,
            nom::character::complete::char('@'),
        )),
        nom::branch::alt((
            parse_tag_text,
            parse_tag_param,
            parse_example,
            parse_tag_unknown,
        )),
    )(input)
}

/// Parses the input as if it was a text tag, such as `@name`, `@description`, etc.
fn parse_tag_text(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::combinator::map(
        nom::sequence::pair(
            nom::branch::alt((
                nom::bytes::complete::tag("author"),
                nom::bytes::complete::tag("cmd"),
                nom::bytes::complete::tag("alias"),
                nom::bytes::complete::tag("description"),
                nom::bytes::complete::tag("default"),
                nom::bytes::complete::tag("help"),
                nom::bytes::complete::tag("name"),
                nom::bytes::complete::tag("version"),
                nom::bytes::complete::tag("rule"),
                nom::bytes::complete::tag("sub"),
            )),
            parse_tail,
        ),
        |(tag, text)| {
            let text = text.to_string();

            Some(match tag {
                "author" => Data::Author(text.split(',').map(|v| v.trim().to_string()).collect()),
                "cmd" => Data::Cmd(text),
                "alias" => Data::Alias(text),
                "description" => Data::Description(text),
                "default" => Data::Default(text),
                "help" => Data::Help(text),
                "name" => Data::Name(text),
                "version" => Data::Version(text),
                "rule" => Data::Rule(text),
                "sub" => Data::Subcommand(text),
                _ => unreachable!(),
            })
        },
    )(input)
}

/// Parses the input as if it was the tail of a line, getting everything after the first space,
/// and checking that we haven't reached the EOF.
fn parse_tail(input: &str) -> nom::IResult<&str, &str> {
    nom::branch::alt((
        nom::combinator::eof,
        nom::sequence::preceded(
            nom::character::complete::space0,
            nom::branch::alt((
                nom::combinator::eof,
                nom::combinator::map(nom::combinator::rest, |v: &str| v.trim()),
            )),
        ),
    ))(input)
}

/// Parses the input as if it was an unknown tag.
fn parse_tag_unknown(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::combinator::map(parse_word, |v| Some(Data::Unknown(v.to_string())))(input)
}

/// Parses the input as if it was a word consisting of alphanumeric characters, underscores, or dashes.
fn parse_word(input: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_while1(is_name_char)(input)
}

/// Parses the input as if it was a function.
fn parse_fn(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::combinator::map(
        nom::branch::alt((parse_fn_keyword, parse_fn_no_keyword)),
        |v| Some(Data::Func(v.to_string())),
    )(input)
}

/// Parses the intput as if it was a function with the `function` keyword.
fn parse_fn_keyword(input: &str) -> nom::IResult<&str, &str> {
    nom::sequence::preceded(
        nom::sequence::tuple((
            nom::character::complete::space0,
            nom::bytes::complete::tag("function"),
            nom::character::complete::space1,
        )),
        parse_fn_name,
    )(input)
}

/// Parses the input as if it was a function name.
fn parse_fn_name(input: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_while1(is_not_fn_name_char)(input)
}

/// Parses the input as if it was a function without the `function` keyword.
fn parse_fn_no_keyword(input: &str) -> nom::IResult<&str, &str> {
    nom::sequence::preceded(
        nom::character::complete::space0,
        nom::sequence::terminated(
            parse_fn_name,
            nom::sequence::tuple((
                nom::character::complete::space0,
                nom::character::complete::char('('),
                nom::character::complete::space0,
                nom::character::complete::char(')'),
            )),
        ),
    )(input)
}

/// Parse the input as if it was a tag parameter like `@option rest...`, `@flag rest...`, or `@arg rest...`.
fn parse_tag_param(input: &str) -> nom::IResult<&str, Option<Data>> {
    let check = nom::combinator::peek(nom::branch::alt((
        nom::bytes::complete::tag("flag"),
        nom::bytes::complete::tag("option"),
        nom::bytes::complete::tag("arg"),
    )));

    let arg = nom::branch::alt((
        nom::combinator::map(
            nom::sequence::preceded(
                nom::sequence::pair(
                    nom::bytes::complete::tag("flag"),
                    nom::character::complete::space1,
                ),
                parse_flag_param,
            ),
            |param| Some(Data::Flag(param)),
        ),
        nom::combinator::map(
            nom::sequence::preceded(
                nom::sequence::pair(
                    nom::bytes::complete::tag("option"),
                    nom::character::complete::space1,
                ),
                parse_option_param,
            ),
            |param| Some(Data::Option(param)),
        ),
        nom::combinator::map(
            nom::sequence::preceded(
                nom::sequence::pair(
                    nom::bytes::complete::tag("arg"),
                    nom::character::complete::space1,
                ),
                parse_arg_param,
            ),
            |param| Some(Data::PositionalArgument(param)),
        ),
    ));

    nom::sequence::preceded(
        check,
        nom::branch::alt((arg, nom::combinator::success(None))),
    )(input)
}

/// Parses the input as if it was a flag parameter like `@flag -h --help <description>`.
fn parse_flag_param(input: &str) -> nom::IResult<&str, param::Flag> {
    nom::combinator::map(
        nom::sequence::tuple((
            parse_short,
            nom::sequence::preceded(
                nom::sequence::pair(
                    nom::character::complete::space0,
                    nom::bytes::complete::tag("--"),
                ),
                parse_param_name,
            ),
            parse_tail,
        )),
        |(short, data, description)| param::Flag::new(data, description, short),
    )(input)
}

/// Parses the input as if it was an on option parameter like `@option --help <description>`.
fn parse_option_param(input: &str) -> nom::IResult<&str, param::Option> {
    nom::combinator::map(
        nom::sequence::tuple((
            parse_short,
            nom::sequence::preceded(
                nom::sequence::pair(
                    nom::character::complete::space0,
                    nom::bytes::complete::tag("--"),
                ),
                nom::branch::alt((
                    parse_param_choices_default,
                    parse_param_choices_required,
                    parse_param_choices_multiple,
                    parse_param_choices,
                    parse_param_assign,
                    parse_param_mark,
                )),
            ),
            parse_value_notation,
            parse_tail,
        )),
        |(short, data, value_notation, description)| {
            param::Option::new(
                data,
                description,
                short,
                value_notation.map(|v| v.to_string()),
            )
        },
    )(input)
}

/// Parses the input as if it was a positional parameter like `@arg name <description>`.
fn parse_arg_param(input: &str) -> nom::IResult<&str, param::PositionalArgument> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::branch::alt((
                parse_param_choices_default,
                parse_param_choices_required,
                parse_param_choices_multiple,
                parse_param_choices,
                parse_param_assign,
                parse_param_mark,
            )),
            parse_value_notation,
            parse_tail,
        )),
        |(data, value_notation, description)| {
            param::PositionalArgument::new(data, description, value_notation.map(|v| v.to_string()))
        },
    )(input)
}

/// Parses the input as if it was a value notation in the form of `<VALUE_NOTATION>`.
fn parse_value_notation(input: &str) -> nom::IResult<&str, Option<&str>> {
    nom::combinator::opt(nom::sequence::preceded(
        nom::character::complete::space0,
        nom::sequence::delimited(
            nom::character::complete::char('<'),
            nom::bytes::complete::take_while1(|c: char| c.is_ascii_uppercase() || c == '_'),
            nom::character::complete::char('>'),
        ),
    ))(input)
}

/// Parses the input as if was marked as `required` or `multiple`. E.g. `@param!`, `@param*`, `@param+`.
fn parse_param_mark(input: &str) -> nom::IResult<&str, param::Data> {
    nom::branch::alt((
        nom::combinator::map(
            nom::sequence::terminated(parse_param_name, nom::bytes::complete::tag("!")),
            |mut data| {
                data.required = true;
                data
            },
        ),
        nom::combinator::map(
            nom::sequence::terminated(parse_param_name, nom::bytes::complete::tag("*")),
            |mut data| {
                data.multiple = true;
                data
            },
        ),
        nom::combinator::map(
            nom::sequence::terminated(parse_param_name, nom::bytes::complete::tag("+")),
            |mut data| {
                data.required = true;
                data.multiple = true;
                data
            },
        ),
        parse_param_name,
    ))(input)
}

/// Parses the input as if it was a value notation like `str=value`.
fn parse_param_assign(input: &str) -> nom::IResult<&str, param::Data> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            parse_param_name,
            nom::character::complete::char('='),
            parse_default_value,
        ),
        |(mut data, value)| {
            data.default = Some(value.to_string());
            data
        },
    )(input)
}

/// Parses the input as if it was a value notation like `str[a|b|c]`.
fn parse_param_choices(input: &str) -> nom::IResult<&str, param::Data> {
    nom::combinator::map(
        nom::sequence::pair(
            parse_param_name,
            nom::sequence::delimited(
                nom::character::complete::char('['),
                parse_choices,
                nom::character::complete::char(']'),
            ),
        ),
        |(mut data, (choices, default))| {
            data.choices = Some(choices.iter().map(|v| v.to_string()).collect());
            data.default = default.map(|v| v.to_string());
            data
        },
    )(input)
}

/// Parses the input as if it was a delimited multiple option value like `[a|b|c]`.
fn parse_delimited_choices(input: &str) -> nom::IResult<&str, (Vec<&str>, Option<&str>)> {
    nom::sequence::delimited(
        nom::character::complete::char('['),
        parse_choices,
        nom::character::complete::char(']'),
    )(input)
}

/// Parses the input as if it was a value notation with a multiple or multiple and required mark
/// like `str*[a|b|c]` or `str+[a|b|c]`.
fn parse_param_choices_multiple(input: &str) -> nom::IResult<&str, param::Data> {
    nom::branch::alt((
        nom::combinator::map(
            nom::sequence::pair(
                nom::sequence::terminated(parse_param_name, nom::character::complete::char('*')),
                parse_delimited_choices,
            ),
            |(mut data, (choices, default))| {
                data.choices = Some(choices.iter().map(|v| v.to_string()).collect());
                data.multiple = true;
                data.default = default.map(|v| v.to_string());
                data
            },
        ),
        nom::combinator::map(
            nom::sequence::pair(
                nom::sequence::terminated(parse_param_name, nom::character::complete::char('+')),
                parse_delimited_choices,
            ),
            |(mut data, (choices, default))| {
                data.choices = Some(choices.iter().map(|v| v.to_string()).collect());
                data.multiple = true;
                data.required = true;
                data.default = default.map(|v| v.to_string());
                data
            },
        ),
    ))(input)
}

/// Parses the input as if it was a value notation with a default value like `str![=a|b|c]`.
fn parse_param_choices_required(input: &str) -> nom::IResult<&str, param::Data> {
    nom::combinator::map(
        nom::sequence::pair(
            nom::sequence::terminated(parse_param_name, nom::character::complete::char('!')),
            parse_delimited_choices,
        ),
        |(mut data, (choices, default))| {
            data.choices = Some(choices.iter().map(|v| v.to_string()).collect());
            data.required = true;
            data.default = default.map(|v| v.to_string());
            data
        },
    )(input)
}

/// Parses the input as if it was a value notation with a default value like `str[=a|b|c]`.
fn parse_param_choices_default(input: &str) -> nom::IResult<&str, param::Data> {
    nom::combinator::map(
        nom::sequence::pair(
            parse_param_name,
            nom::sequence::delimited(
                nom::character::complete::char('['),
                parse_choices_default,
                nom::character::complete::char(']'),
            ),
        ),
        |(mut data, (choices, default))| {
            data.choices = Some(choices.iter().map(|v| v.to_string()).collect());
            data.default = default.map(|v| v.to_string());
            data
        },
    )(input)
}

/// Parses the input as if it was a list of possible values like `=a|b|c`.
fn parse_choices_default(input: &str) -> nom::IResult<&str, (Vec<&str>, Option<&str>)> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::character::complete::char('='),
            parse_choice_value,
            nom::multi::many1(nom::sequence::preceded(
                nom::character::complete::char('|'),
                parse_choice_value,
            )),
        )),
        |(_, head, tail)| {
            let mut choices = vec![head];
            choices.extend(tail);
            (choices, Some(head))
        },
    )(input)
}

/// Parses the input as if it had a default value like `str=value` or `str="value"`.
fn parse_default_value(input: &str) -> nom::IResult<&str, &str> {
    nom::branch::alt((
        parse_quoted_string,
        nom::bytes::complete::take_till(is_default_value_terminate),
    ))(input)
}

/// Parses the input as if it was a list of possible values like `a|b|c`.
fn parse_choices(input: &str) -> nom::IResult<&str, (Vec<&str>, Option<&str>)> {
    nom::combinator::map(
        nom::multi::separated_list1(nom::character::complete::char('|'), parse_choice_value),
        |choices| (choices, None),
    )(input)
}

/// Parses the input as if it was a value like `str` or `"str"`.
fn parse_choice_value(input: &str) -> nom::IResult<&str, &str> {
    if input.starts_with('=') {
        return nom::combinator::fail(input);
    }
    nom::branch::alt((
        parse_quoted_string,
        nom::bytes::complete::take_till(is_choice_value_terminate),
    ))(input)
}

fn parse_quoted_string(input: &str) -> nom::IResult<&str, &str> {
    let single = nom::sequence::delimited(
        nom::character::complete::char('\''),
        nom::branch::alt((
            nom::bytes::complete::escaped(
                nom::character::streaming::none_of("\\\'"),
                '\\',
                nom::character::complete::char('\''),
            ),
            nom::bytes::complete::tag(""),
        )),
        nom::character::complete::char('\''),
    );

    let double = nom::sequence::delimited(
        nom::character::complete::char('"'),
        nom::branch::alt((
            nom::bytes::complete::escaped(
                nom::character::streaming::none_of("\\\""),
                '\\',
                nom::character::complete::char('"'),
            ),
            nom::bytes::complete::tag(""),
        )),
        nom::character::complete::char('"'),
    );

    nom::branch::alt((single, double))(input)
}

/// Returns true if the character is a `|` or `]` which terminates a choice value.
pub fn is_choice_value_terminate(c: char) -> bool {
    c == '|' || c == ']'
}

/// Returns true if the character is a whitespace which terminates a default value.
pub fn is_default_value_terminate(c: char) -> bool {
    c.is_whitespace()
}

/// Parses the input as if it was a short option like `-h`.
fn parse_short(input: &str) -> nom::IResult<&str, Option<char>> {
    let short = nom::sequence::delimited(
        nom::character::complete::char('-'),
        nom::character::complete::satisfy(|c| c.is_ascii_alphanumeric()),
        nom::combinator::peek(nom::character::complete::space1),
    );

    nom::combinator::opt(short)(input)
}

/// Parses the input as if it was a parameter name like `--help`.
fn parse_param_name(input: &str) -> nom::IResult<&str, param::Data> {
    nom::combinator::map(parse_name, param::Data::new)(input)
}

/// Parses the input as if it was a string of ascii alphanumeric text, plus `-` or `_`.
fn parse_name(input: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_while1(is_name_char)(input)
}

/// Returns true if the character is not a valid bash function name character.
fn is_not_fn_name_char(c: char) -> bool {
    !matches!(
        c,
        ' ' | '\t'
            | '"'
            | '\''
            | '`'
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | '<'
            | '>'
            | '$'
            | '&'
            | '\\'
            | ';'
            | '|'
    )
}

/// Returns true if the character is an ascii alphanumeric character, underscore, or dash.
fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_token {
        ($comment:literal, Ignore) => {
            assert_eq!(
                parse_line($comment).unwrap(),
                Some(Data::Unknown($comment.to_string()))
            )
        };
        ($comment:literal, Error) => {
            assert_eq!(parse_line($comment).unwrap(), None)
        };
        ($comment: literal, $kind:expr) => {
            assert_eq!(parse_line($comment).unwrap(), Some($kind))
        };
    }

    #[test]
    fn test_parse_line() {
        assert_token!(
            "# @author Foo, Bar, Baz",
            Data::Author(vec!(
                "Foo".to_string(),
                "Bar".to_string(),
                "Baz".to_string()
            ))
        );
        assert_token!("# @description foo", Data::Description("foo".to_string()));
        assert_token!("# @version 1", Data::Version("1".to_string()));
        assert_token!("# @help foo", Data::Help("foo".to_string()));
        assert_token!("# @name foo", Data::Name("foo".to_string()));
        assert_token!("# @default foo", Data::Default("foo".to_string()));
        assert_token!("# @something foo", Data::Unknown("something".to_string()));
        assert_token!(
            "# @flag --flag A flag",
            Data::Flag(param::Flag {
                name: "flag".to_string(),
                description: "A flag".to_string(),
                ..Default::default()
            })
        );
        assert_token!(
            "# @flag -f --flag A flag with a short and long name",
            Data::Flag(param::Flag {
                name: "flag".to_string(),
                short: Some('f'),
                description: "A flag with a short and long name".to_string(),
            })
        );
        assert_token!(
            "# @option --option An option",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option".to_string(),
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option <VALUE_NOTATION> An option with a specific value notation",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option with a specific value notation".to_string(),
                value_notation: Some("VALUE_NOTATION".to_string()),
                ..Default::default()
            })
        );

        assert_token!(
            "# @option -o --option An option with a short and long version",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option with a short and long version".to_string(),
                short: Some('o'),
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option! A required option",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "A required option".to_string(),
                required: true,
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option=foo An option with a default value",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option with a default value".to_string(),
                default: Some("foo".to_string()),
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option=\"foo bar\" An option with a default multi word value",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option with a default multi word value".to_string(),
                default: Some("foo bar".to_string()),
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option* An option that takes multiple values",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option that takes multiple values".to_string(),
                multiple: true,
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option+ An option that takes multiple values and its required",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option that takes multiple values and its required".to_string(),
                multiple: true,
                required: true,
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option[a|b|c] An option that supports predefined values",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option that supports predefined values".to_string(),
                choices: Some(vec!("a".to_string(), "b".to_string(), "c".to_string())),
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option[=a|b|c] An option that supports predefined values and has a default",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option that supports predefined values and has a default".to_string(),
                choices: Some(vec!("a".to_string(), "b".to_string(), "c".to_string())),
                default: Some("a".to_string()),
                ..Default::default()
            })
            );
        assert_token!(
            "# @option --option![a|b|c] An option that supports predefined values and its required",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option that supports predefined values and its required"
                    .to_string(),
                choices: Some(vec!("a".to_string(), "b".to_string(), "c".to_string())),
                required: true,
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option*[a|b|c] An option that supports predefined values and its multiple",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option that supports predefined values and its multiple"
                    .to_string(),
                choices: Some(vec!("a".to_string(), "b".to_string(), "c".to_string())),
                multiple: true,
                ..Default::default()
            })
        );
        assert_token!(
            "# @option --option+[a|b|c] An option that supports predefined values and its multiple and required",
            Data::Option(param::Option {
                name: "option".to_string(),
                description: "An option that supports predefined values and its multiple and required".to_string(),
                choices: Some(vec!("a".to_string(), "b".to_string(), "c".to_string())),
                multiple: true,
                required: true,
                ..Default::default()
            })
            );

        assert_token!(
            "# @arg positional_argument A positional argument",
            Data::PositionalArgument(param::PositionalArgument {
                name: "positional_argument".to_string(),
                description: "A positional argument".to_string(),
                ..Default::default()
            })
        );
        assert_token!(
            "# @arg positional_argument! A required option",
            Data::PositionalArgument(param::PositionalArgument {
                name: "positional_argument".to_string(),
                description: "A required option".to_string(),
                required: true,
                ..Default::default()
            })
        );
        assert_token!(
            "# @arg positional_argument=foo A positional argument with a default value",
            Data::PositionalArgument(param::PositionalArgument {
                name: "positional_argument".to_string(),
                description: "A positional argument with a default value".to_string(),
                default: Some("foo".to_string()),
                ..Default::default()
            })
        );
        assert_token!(
            "# @arg positional_argument=\"foo bar\" A positional argument with a default multi word value",
            Data::PositionalArgument(param::PositionalArgument {
                name: "positional_argument".to_string(),
                description: "A positional argument with a default multi word value".to_string(),
                default: Some("foo bar".to_string()),
                ..Default::default()
            })
            );
        assert_token!(
            "# @arg positional_argument[a|b|c] A positional argument that supports predefined values",
            Data::PositionalArgument(param::PositionalArgument {
                name: "positional_argument".to_string(),
                description: "A positional argument that supports predefined values".to_string(),
                choices: Some(vec!("a".to_string(), "b".to_string(), "c".to_string())),
                ..Default::default()
            })
            );
        assert_token!(
            "# @arg positional_argument[=a|b|c] A positional argument that supports predefined values and has a default",
            Data::PositionalArgument(param::PositionalArgument {
                name: "positional_argument".to_string(),
                description: "A positional argument that supports predefined values and has a default".to_string(),
                choices: Some(vec!("a".to_string(), "b".to_string(), "c".to_string())),
                default: Some("a".to_string()),
                ..Default::default()
            })
            );
        assert_token!(
            "# @arg positional_argument![a|b|c] A positional argument that supports predefined values and its required",
            Data::PositionalArgument(param::PositionalArgument {
                name: "positional_argument".to_string(),
                description: "A positional argument that supports predefined values and its required".to_string(),
                choices: Some(vec!("a".to_string(), "b".to_string(), "c".to_string())),
                required: true,
                ..Default::default()
            })
            );
        assert_token!(
            "# @arg positional_argument <VALUE_NOTATION> A positional argument with a specific value notation.",
            Data::PositionalArgument(param::PositionalArgument {
                name: "positional_argument".to_string(),
                description: "A positional argument with a specific value notation.".to_string(),
                value_notation: Some("VALUE_NOTATION".to_string()),
                ..Default::default()
            })
            );

        assert_token!("foo()", Data::Func("foo".to_string()));
        assert_token!("foo ()", Data::Func("foo".to_string()));
        assert_token!("foo  ()", Data::Func("foo".to_string()));
        assert_token!("foo  ( )", Data::Func("foo".to_string()));
        assert_token!(" foo  ( )", Data::Func("foo".to_string()));
        assert_token!(" foo-bar ( )", Data::Func("foo-bar".to_string()));
        assert_token!(" foo_bar ( )", Data::Func("foo_bar".to_string()));
        assert_token!(" foo:bar ( )", Data::Func("foo:bar".to_string()));
        assert_token!(" foo.bar ( )", Data::Func("foo.bar".to_string()));
        assert_token!(" foo@bar ( )", Data::Func("foo@bar".to_string()));
        assert_token!("function foo", Data::Func("foo".to_string()));
        assert_token!("function foo_bar", Data::Func("foo_bar".to_string()));
        assert_token!("function foo-bar", Data::Func("foo-bar".to_string()));
        assert_token!("function foo:bar", Data::Func("foo:bar".to_string()));
        assert_token!("function foo.bar", Data::Func("foo.bar".to_string()));
        assert_token!("function foo@bar", Data::Func("foo@bar".to_string()));
        assert_token!("#!/bin/bash", Data::SheBang("#!/bin/bash".to_string()));
        assert_token!("foo=bar", Data::Line("foo=bar".to_string()));
        assert_token!("# @alias foo", Data::Alias("foo".to_string()));
        assert_token!(
            "# @example Something awesome $ something awesome",
            Data::Example(param::Example {
                description: "Something awesome".to_string(),
                command: "something awesome".to_string(),
            })
        );
        assert_token!(
            "# @example Nothing $",
            Data::Example(param::Example {
                description: "Nothing".to_string(),
                command: "".to_string(),
            })
        );
        assert_token!("# @flag -f", Ignore);
        assert_token!("# @option -foo![=a|b]", Ignore);
        assert_token!(
            "# @rule no-first-option-help",
            Data::Rule("no-first-option-help".to_string())
        );
    }
}
