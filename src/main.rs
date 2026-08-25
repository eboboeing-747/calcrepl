use std::{collections::binary_heap::Iter, fmt::Display};

enum TokenType {
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

struct Token<'a> {
    _type: TokenType,
    line: i32,
    lexeme: &'a str,
}

fn main() {
    let source_file: String = String::from("( 1 + 1 )");

    let left_paren: Token = Token {
        _type: TokenType::LeftParen,
        line: 0,
        lexeme: &source_file[0..1],
    };

    println!("[line {0}] '{1}' TokenType::{2}", left_paren.line, left_paren.lexeme, left_paren._type);
    // println!("[line {0}] '{1}'", token.line, token.lexeme);

    let plus = Token {
        _type: TokenType::Plus,
        line: 0,
        lexeme: &source_file[4..5],
    };

    println!("[line {0}] '{1}' TokenType::{2}", plus.line, plus.lexeme, plus._type);
}
