use std::fmt::Display;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Eof,
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
            Self::Eof => write!(f, "EOF"),
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

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub _type: TokenType,
    pub lexeme: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(_type: TokenType, lexeme: &'a str) -> Self {
        return Token { _type, lexeme };
    }

    pub fn new_eof() -> Self {
        return Token { _type: TokenType::Eof, lexeme: "" };
    }
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Token) -> bool {
        return self._type == other._type && self.lexeme == other.lexeme;
    }
}
