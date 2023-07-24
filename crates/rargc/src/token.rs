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
}
