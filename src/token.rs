#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token<'a> {
    Eof,
    Equal(&'a str),
    Error(&'a str),
    Identifier(&'a str),
    LeftParen(&'a str),
    Minus(&'a str),
    Number(&'a str),
    Plus(&'a str),
    RightParen(&'a str),
    Semicolon(&'a str),
    Slash(&'a str),
    Star(&'a str),
}
