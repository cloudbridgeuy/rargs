use color_eyre::eyre::{self, Result};

pub type Position = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub data: Data,
    pub position: Position,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Data {
    Name(String),
    Description(String),
    Author(Vec<String>),
    Version(String),
    Help(String),
    Func(String),
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
            Ok(None) => {
                println!("Add a debug! trace here");
            }
            Err(e) => return Err(e),
        }
    }

    Ok(tokens)
}

/// Parse a line into a token.
pub fn parse_line(line: &str) -> Result<Option<Data>> {
    let maybe = nom::branch::alt((
        nom::combinator::map(nom::branch::alt((parse_tag, parse_fn)), Some),
        nom::combinator::success(None),
    ))(line);

    match maybe {
        Ok((something, maybe_token)) => {
            println!("something: {:?}", something);
            if let Some(maybe_data) = maybe_token {
                if let Some(data) = maybe_data {
                    Ok(Some(data))
                } else {
                    Err(eyre::format_err!("syntax error on line \"{}\"", line))
                }
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

/// Parses an input as if it was a bash comment with a tag such as
/// `# @name rest...`, `# @description rest...`, etc.
fn parse_tag(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::sequence::preceded(
        nom::sequence::tuple((
            nom::multi::many1(nom::character::complete::char('#')),
            nom::character::complete::space0,
            nom::character::complete::char('@'),
        )),
        nom::branch::alt((parse_tag_text, parse_tag_unknown)),
    )(input)
}

/// Parses the input as if it was a text tag, such as `@name`, `@description`, etc.
fn parse_tag_text(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::combinator::map(
        nom::sequence::pair(
            nom::branch::alt((
                nom::bytes::complete::tag("description"),
                nom::bytes::complete::tag("author"),
                nom::bytes::complete::tag("version"),
                nom::bytes::complete::tag("help"),
                nom::bytes::complete::tag("name"),
            )),
            parse_tail,
        ),
        |(tag, text)| {
            let text = text.to_string();

            Some(match tag {
                "description" => Data::Description(text),
                "author" => Data::Author(text.split(',').map(|v| v.trim().to_string()).collect()),
                "version" => Data::Version(text),
                "help" => Data::Help(text),
                "name" => Data::Name(text),
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

/// Parses the input as if it were a function.
fn parse_fn(input: &str) -> nom::IResult<&str, Option<Data>> {
    nom::combinator::map(
        nom::branch::alt((parse_fn_keyword, parse_fn_no_keyword)),
        |v| Some(Data::Func(v.to_string())),
    )(input)
}

/// Parses a function with the `function` keyword.
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

/// Parses a function name.
fn parse_fn_name(input: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_while1(is_not_fn_name_char)(input)
}

/// Parses a function without the `function` keyword.
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
