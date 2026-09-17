use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token<'a> {
    Eof,
    Error(&'a str),
    LeftParen(&'a str),
    Minus(&'a str),
    Number(&'a str),
    Plus(&'a str),
    RightParen(&'a str),
    Slash(&'a str),
    Star(&'a str),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eof => write!(f, "Eof"),
            Self::Error(lexeme) => write!(f, "{}", lexeme),
            Self::LeftParen(lexeme) => write!(f, "{}", lexeme),
            Self::Minus(lexeme) => write!(f, "{}", lexeme),
            Self::Number(lexeme) => write!(f, "{}", lexeme),
            Self::Plus(lexeme) => write!(f, "{}", lexeme),
            Self::RightParen(lexeme) => write!(f, "{}", lexeme),
            Self::Slash(lexeme) => write!(f, "{}", lexeme),
            Self::Star(lexeme) => write!(f, "{}", lexeme),
        }
    }
}

// impl PartialEq for Token<'_> {
//     fn eq(&self, other: &Token) -> bool {
//         return self._type == other._type && self.lexeme == other.lexeme;
//     }
// }
