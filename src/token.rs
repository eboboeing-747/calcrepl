use std::fmt::Display;

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

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eof => write!(f, "Eof"),
            Self::Equal(lexeme) => write!(f, "{}", lexeme),
            Self::Error(lexeme) => write!(f, "{}", lexeme),
            Self::Identifier(lexeme) => write!(f, "{}", lexeme),
            Self::LeftParen(lexeme) => write!(f, "{}", lexeme),
            Self::Minus(lexeme) => write!(f, "{}", lexeme),
            Self::Number(lexeme) => write!(f, "{}", lexeme),
            Self::Plus(lexeme) => write!(f, "{}", lexeme),
            Self::RightParen(lexeme) => write!(f, "{}", lexeme),
            Self::Semicolon(lexeme) => write!(f, "{}", lexeme),
            Self::Slash(lexeme) => write!(f, "{}", lexeme),
            Self::Star(lexeme) => write!(f, "{}", lexeme),
        }
    }
}
