use std::fmt::Display;

#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(Debug)]
pub struct Token<'a> {
    pub _type: TokenType,
    pub lexeme: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(_type: TokenType, lexeme: &'a str) -> Self {
        return Token { _type, lexeme };
    }
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Token) -> bool {
        return self._type == other._type && self.lexeme == other.lexeme;
    }
}
