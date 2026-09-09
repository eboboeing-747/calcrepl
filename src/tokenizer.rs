use std::str::CharIndices;
use std::iter::Peekable;
use crate::token::{TokenType, Token};

pub struct Tokenizer<'a> {
    start: usize,
    current: usize,
    source_file: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source_file: &'a str) -> Self {
        return Tokenizer {
            start: 0,
            current: 0,
            source_file,
            chars: source_file.char_indices().peekable(),
        };
    }

    fn peek(&mut self) -> Option<char> {
        return match self.chars.peek() {
            Some((_i, c)) => Some(*c),
            None => None,
        };
    }

    fn advance(&mut self) -> Option<char> {
        let (i, c) = match self.chars.next() {
            Some((i, c)) => (i, c),
            None => return None,
        };

        // println!("[advance]{}", i);
        self.current = i + c.len_utf8();
        return Some(c);
    }

    fn skip_whitespace(&mut self) -> () {
        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => break,
            };
            if c == ' ' {
                self.advance();
                continue;
            }
            break;
        }
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek().unwrap_or('\0').is_digit(10) { self.advance(); }
        return self.make_token(TokenType::Number);
    }

    fn make_token(&mut self, _type: TokenType) -> Token<'a> {
        let token = Token {
            _type: _type,
            lexeme: &self.source_file[self.start..self.current]
        };

        // println!("[make_token]{} {}", self.start, self.current);
        return token;
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;
        let c = match self.advance() {
            Some(c) => c,
            None => return self.make_token(TokenType::EOF),
        };

        if c.is_digit(10) {
            return self.number();
        }

        match c {
            '+' => return self.make_token(TokenType::Plus),
            '-' => return self.make_token(TokenType::Minus),
            '*' => return self.make_token(TokenType::Star),
            '/' => return self.make_token(TokenType::Slash),
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            _ => return self.make_token(TokenType::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let mut tokenizer = Tokenizer::new("");
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }

    #[test]
    fn single_whitespace_string() {
        let mut tokenizer = Tokenizer::new(" ");
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }

    #[test]
    fn long_whitespace_string() {
        let mut tokenizer = Tokenizer::new("         ");
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }

    #[test]
    fn single_symbol_number() {
        let mut tokenizer = Tokenizer::new("1");
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }

    #[test]
    fn single_symbol_tokens() {
        let mut tokenizer = Tokenizer::new("(1+1)");
        assert_eq!(tokenizer.scan_token()._type, TokenType::LeftParen);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Plus);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::RightParen);
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }

    #[test]
    fn spaced_single_symbol_tokens() {
        let mut tokenizer = Tokenizer::new("( 1 + 1 )");
        assert_eq!(tokenizer.scan_token()._type, TokenType::LeftParen);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Plus);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::RightParen);
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }

    #[test]
    fn long_spaced_single_symbol_tokens() {
        let mut tokenizer = Tokenizer::new("(    1     +1    )");
        assert_eq!(tokenizer.scan_token()._type, TokenType::LeftParen);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Plus);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::RightParen);
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }

    #[test]
    fn long_number() {
        let mut tokenizer = Tokenizer::new("123456");
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }

    #[test]
    fn expression() {
        let mut tokenizer = Tokenizer::new("123456 + 1  / (984 - 12)");
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Plus);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Slash);
        assert_eq!(tokenizer.scan_token()._type, TokenType::LeftParen);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Minus);
        assert_eq!(tokenizer.scan_token()._type, TokenType::Number);
        assert_eq!(tokenizer.scan_token()._type, TokenType::RightParen);
        assert_eq!(tokenizer.scan_token()._type, TokenType::EOF);
    }
}
