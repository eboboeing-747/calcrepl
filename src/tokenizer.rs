use std::str::CharIndices;
use std::iter::Peekable;
use crate::token::Token;

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
            return Token::Number(self.lexeme());
        }

        if !self.peek().unwrap_or('\0').is_digit(10) {
            return Token::Error(self.lexeme());
        }
        while self.peek().unwrap_or('\0').is_digit(10) { self.advance(); }
        return Token::Number(self.lexeme());
    }

    fn lexeme(&mut self) -> &'a str {
        return &self.source_file[self.start..self.current];
    }

    pub fn scan_token(& mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;
        let c = match self.advance() {
            Some(c) => c,
            None => return Token::Eof,
        };

        if c.is_digit(10) {
            return self.number();
        }

        return match c {
            '+' => Token::Plus(self.lexeme()),
            '-' => Token::Minus(self.lexeme()),
            '*' => Token::Star(self.lexeme()),
            '/' => Token::Slash(self.lexeme()),
            '(' => Token::LeftParen(self.lexeme()),
            ')' => Token::RightParen(self.lexeme()),
            _ => Token::Error(self.lexeme()),
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
            res.push(token);
            if token == Token::Eof { break; }
        }
        return res;
    }

    #[test]
    fn empty_string() {
        assert_eq!(
            vec![Token::Eof],
            vectorize(""),
        );
    }

    #[test]
    fn single_whitespace_string() {
        assert_eq!(
            vec![Token::Eof],
            vectorize(" "),
        );
    }

    #[test]
    fn long_whitespace_string() {
        assert_eq!(
            vec![Token::Eof],
            vectorize("         "),
        )
    }

    #[test]
    fn single_symbol_number() {
        assert_eq!(
            vec![
                Token::Number("1"),
                Token::Eof,
            ],
            vectorize("1")
        );
    }

    #[test]
    fn single_symbol_tokens() {
        assert_eq!(
            vec![
                Token::LeftParen("("),
                Token::Number("1"),
                Token::Plus("+"),
                Token::Number("1"),
                Token::RightParen(")"),
                Token::Eof,
            ],
            vectorize("(1+1)"),
        );
    }

    #[test]
    fn spaced_single_symbol_tokens() {
        assert_eq!(
            vec![
                Token::LeftParen("("),
                Token::Number("1"),
                Token::Plus("+"),
                Token::Number("1"),
                Token::RightParen(")"),
                Token::Eof,
            ],
            vectorize("( 1 + 1 )"),
        );
    }

    #[test]
    fn long_spaced_single_symbol_tokens() {
        assert_eq!(
            vec![
                Token::LeftParen("("),
                Token::Number("1"),
                Token::Plus("+"),
                Token::Number("1"),
                Token::RightParen(")"),
                Token::Eof,
            ],
            vectorize("(    1     +1    )"),
        );
    }

    #[test]
    fn long_number() {
        assert_eq!(
            vec![
                Token::Number("123456"),
                Token::Eof,
            ],
            vectorize("123456"),
        );
    }

    #[test]
    fn expression() {
        assert_eq!(
            vec![
                Token::Number("123456"),
                Token::Plus("+"),
                Token::Number("1"),
                Token::Slash("/"),
                Token::LeftParen("("),
                Token::Number("984"),
                Token::Minus("-"),
                Token::Number("12"),
                Token::RightParen(")"),
                Token::Eof,
            ],
            vectorize("123456 + 1  / (984 - 12)"),
        );
    }

    #[test]
    fn float() {
        assert_eq!(
            vec![
                Token::Number("1.1"),
                Token::Eof,
            ],
            vectorize("1.1"),
        );
    }

    #[test]
    fn long_float() {
        assert_eq!(
            vec![
                Token::Number("1245.123142"),
                Token::Eof,
            ],
            vectorize("1245.123142"),
        );
    }

    #[test]
    fn paren_invalid_float() {
        assert_eq!(
            vec![
                Token::LeftParen("("),
                Token::LeftParen("("),
                Token::Error("."),
                Token::Number("23423"),
                Token::RightParen(")"),
                Token::RightParen(")"),
                Token::Eof,
            ],
            vectorize("((.23423))"),
        );
    }

    #[test]
    fn invalid_float() {
        assert_eq!(
            vec![
                Token::Error("0."),
                Token::Eof,
            ],
            vectorize("0."),
        );
    }

    #[test]
    fn invalid_float_2() {
        assert_eq!(
            vec![
                Token::Error("9888."),
                Token::Eof,
            ],
            vectorize("9888."),
        );
    }
}
