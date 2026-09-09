use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    EOF,
    Error,
    LeftParen,
    Minus,
    Number,
    Plus,
    RightParen,
    Slash,
    Star,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOF => write!(f, "EOF"),
            Self::Error => write!(f, "Error"),
            Self::LeftParen => write!(f, "LeftParen"),
            Self::Minus => write!(f, "Minus"),
            Self::Number => write!(f, "Number"),
            Self::Plus => write!(f, "Plus"),
            Self::RightParen => write!(f, "RightParen"),
            Self::Slash => write!(f, "Slash"),
            Self::Star => write!(f, "Star"),
        }
    }
}

pub struct Token<'a> {
    pub _type: TokenType,
    pub lexeme: &'a str,
}
