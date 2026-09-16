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
        if self.peek().unwrap_or('\0') == '.' {
            self.advance();
        } else {
            return self.make_token(TokenType::Number);
        }
        if !self.peek().unwrap_or('\0').is_digit(10) {
            return self.make_token(TokenType::Error);
        }
        while self.peek().unwrap_or('\0').is_digit(10) { self.advance(); }
        return self.make_token(TokenType::Number);
    }

    fn make_token(&mut self, _type: TokenType) -> Token<'a> {
        let token = Token {
            _type: _type,
            lexeme: &self.source_file[self.start..self.current]
        };

        return token;
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;
        let c = match self.advance() {
            Some(c) => c,
            None => return self.make_token(TokenType::Eof),
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

    fn vectorize<'a>(src: &'a str) -> Vec<Token<'a>> {
        let mut res: Vec<Token<'a>> = Vec::new();
        let mut tokenizer = Tokenizer::new(src);
        loop {
            let token = tokenizer.scan_token();
            let _type = token._type;
            res.push(token);
            if _type == TokenType::Eof { break; }
        }
        return res;
    }

    #[test]
    fn empty_string() {
        assert_eq!(
            vec![Token::new(TokenType::Eof, "")],
            vectorize(""),
        );
    }

    #[test]
    fn single_whitespace_string() {
        assert_eq!(
            vec![Token::new(TokenType::Eof, "")],
            vectorize(" "),
        );
    }

    #[test]
    fn long_whitespace_string() {
        assert_eq!(
            vec![Token::new(TokenType::Eof, "")],
            vectorize("         "),
        )
    }

    #[test]
    fn single_symbol_number() {
        assert_eq!(
            vec![
                Token::new(TokenType::Number, "1"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("1")
        );
    }

    #[test]
    fn single_symbol_tokens() {
        assert_eq!(
            vec![
                Token::new(TokenType::LeftParen, "("),
                Token::new(TokenType::Number, "1"),
                Token::new(TokenType::Plus, "+"),
                Token::new(TokenType::Number, "1"),
                Token::new(TokenType::RightParen, ")"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("(1+1)"),
        );
    }

    #[test]
    fn spaced_single_symbol_tokens() {
        assert_eq!(
            vec![
                Token::new(TokenType::LeftParen, "("),
                Token::new(TokenType::Number, "1"),
                Token::new(TokenType::Plus, "+"),
                Token::new(TokenType::Number, "1"),
                Token::new(TokenType::RightParen, ")"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("( 1 + 1 )"),
        );
    }

    #[test]
    fn long_spaced_single_symbol_tokens() {
        assert_eq!(
            vec![
                Token::new(TokenType::LeftParen, "("),
                Token::new(TokenType::Number, "1"),
                Token::new(TokenType::Plus, "+"),
                Token::new(TokenType::Number, "1"),
                Token::new(TokenType::RightParen, ")"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("(    1     +1    )"),
        );
    }

    #[test]
    fn long_number() {
        assert_eq!(
            vec![
                Token::new(TokenType::Number, "123456"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("123456"),
        );
    }

    #[test]
    fn expression() {
        assert_eq!(
            vec![
                Token::new(TokenType::Number, "123456"),
                Token::new(TokenType::Plus, "+"),
                Token::new(TokenType::Number, "1"),
                Token::new(TokenType::Slash, "/"),
                Token::new(TokenType::LeftParen, "("),
                Token::new(TokenType::Number, "984"),
                Token::new(TokenType::Minus, "-"),
                Token::new(TokenType::Number, "12"),
                Token::new(TokenType::RightParen, ")"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("123456 + 1  / (984 - 12)"),
        );
    }

    #[test]
    fn float() {
        assert_eq!(
            vec![
                Token::new(TokenType::Number, "1.1"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("1.1"),
        );
    }

    #[test]
    fn long_float() {
        assert_eq!(
            vec![
                Token::new(TokenType::Number, "1245.123142"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("1245.123142"),
        );
    }

    #[test]
    fn paren_invalid_float() {
        assert_eq!(
            vec![
                Token::new(TokenType::LeftParen, "("),
                Token::new(TokenType::LeftParen, "("),
                Token::new(TokenType::Error, "."),
                Token::new(TokenType::Number, "23423"),
                Token::new(TokenType::RightParen, ")"),
                Token::new(TokenType::RightParen, ")"),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("((.23423))"),
        );
    }

    #[test]
    fn invalid_float() {
        assert_eq!(
            vec![
                Token::new(TokenType::Error, "0."),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("0."),
        );
    }

    #[test]
    fn invalid_float_2() {
        assert_eq!(
            vec![
                Token::new(TokenType::Error, "9888."),
                Token::new(TokenType::Eof, ""),
            ],
            vectorize("9888."),
        );
    }
}
