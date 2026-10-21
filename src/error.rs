use std::io::stdout;
use std::fmt::Display;
use crossterm::{cursor, execute};
use crossterm::style::{Color, Colors, SetColors, ResetColor};

use crate::token::Token;

enum Direction {
    Back,
    Forth,
}

#[derive(Debug)]
pub struct Error<'a> {
    source_file: &'a str,
    token: Token<'a>,
    #[cfg(not(test))]
    message: String,
    #[cfg(test)]
    pub message: String,
    line: usize,
}

impl<'a> Error<'a> {
    pub fn new(source_file: &'a str, token: Token<'a>,
        message: String, line: usize) -> Self {
        return Error { source_file, token, message, line };
    }

    fn offset(&self, direction: Direction) -> usize {
        return match self.token {
            Token::Eof(l) | Token::Equal(l) | Token::Error(l) |
                Token::Identifier(l) | Token::LeftParen(l) | Token::Minus(l) |
                Token::Number(l) | Token::Plus(l) | Token::RightParen(l) |
                Token::Semicolon(l) | Token::Slash(l) | Token::Star(l) => {
                let offset =
                    l.as_ptr() as usize - self.source_file.as_ptr() as usize;
                match direction {
                    Direction::Back => offset,
                    Direction::Forth => offset + l.len(),
                }
            }
        };
    }

    fn back(&self) -> &str {
        let offset = self.offset(Direction::Back);
        let tmpstr = &self.source_file[..offset];
        let line_start: usize = match tmpstr.rfind('\n') {
            Some(i) => i + 1,
            None => 0,
        };
        return &self.source_file[line_start..offset];
    }

    fn forth(&self) -> &str {
        let offset = self.offset(Direction::Forth);
        let tmpstr = &self.source_file[offset..];
        let line_end: usize = match tmpstr.find('\n') {
            Some(i) => offset + i,
            None => self.source_file.len(),
        };
        return &self.source_file[offset..line_end];
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lexeme = match self.token {
            Token::Eof(l) | Token::Equal(l) | Token::Error(l) |
                Token::Identifier(l) | Token::LeftParen(l) | Token::Minus(l) |
                Token::Number(l) | Token::Plus(l) | Token::RightParen(l) |
                Token::Semicolon(l) | Token::Slash(l) | Token::Star(l) => l,
        };

        let _ = execute!(stdout(),
            SetColors(Colors::new(Color::Blue, Color::Reset)));
        let line = self.line.to_string();
        let _ = write!(f, "{} | ", line);
        let _ = execute!(stdout(), ResetColor);
        let _ = write!(f, "{}", self.back());

        let (start_x, _y) = cursor::position()
            .unwrap_or_else(|_| panic!("failed to get cursor position"));
        let _ = execute!(stdout(),
            SetColors(Colors::new(Color::Red, Color::Reset)));
        let _ = write!(f, "{lexeme}");
        let _ = execute!(stdout(), ResetColor);
        let (end_x, _y) = cursor::position()
            .unwrap_or_else(|_| panic!("failed to get cursor position"));

        let _ = write!(f, "{}\n", self.forth());

        let _ = execute!(stdout(),
            SetColors(Colors::new(Color::Blue, Color::Reset)));
        for _ in 0..line.len() + 1 { let _ = write!(f, " "); }
        let _ = write!(f, "| ");

        let mut offset = start_x as usize - line.len() - 3;
        let mut is_front = false;
        if self.message.len() < offset {
            offset -= self.message.len() + 1;
            is_front = true;
        }
        let _ = execute!(stdout(),
            SetColors(Colors::new(Color::Red, Color::Reset)));
        for _ in 0..offset { let _ = write!(f, " "); }
        if is_front { let _ = write!(f, "{} ", self.message); }

        for _ in start_x..end_x { let _ = write!(f, "^"); }
        if start_x == end_x { let _ = write!(f, "^"); }

        if !is_front { let _ = write!(f, " {}", self.message); }
        let _ = execute!(stdout(), ResetColor);

        return std::result::Result::Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset() {
        let src = "111 + 222";

        let token = Token::Number(&src[..3]);
        assert_eq!(token, Token::Number("111"));
        let error = Error::new(src, token, String::from(""), 1);
        assert_eq!(error.offset(Direction::Back), 0);
        assert_eq!(error.offset(Direction::Forth), 3);

        let token = Token::Plus(&src[4..5]);
        assert_eq!(token, Token::Plus("+"));
        let error = Error::new(src, token, String::from(""), 1);
        assert_eq!(error.offset(Direction::Back), 4);
        assert_eq!(error.offset(Direction::Forth), 5);

        let token = Token::Number(&src[6..]);
        assert_eq!(token, Token::Number("222"));
        let error = Error::new(src, token, String::from(""), 1);
        assert_eq!(error.offset(Direction::Back), 6);
        assert_eq!(error.offset(Direction::Forth), 9);

        let token = Token::Eof(&src[9..]);
        assert_eq!(token, Token::Eof(""));
        let error = Error::new(src, token, String::from(""), 1);
        assert_eq!(error.offset(Direction::Back), 9);
    }

    #[test]
    fn back_no_lf() {
        let src = "1 + 2";

        let token = Token::Number(&src[0..1]);
        let error = Error::new(src, token, String::from(""), 1);
        let back: &str = error.back();
        assert_eq!(back, &src[..0]);

        let token = Token::Plus(&src[2..3]);
        let error = Error::new(src, token, String::from(""), 1);
        let back: &str = error.back();
        assert_eq!(back, &src[..2]);

        let token = Token::Number(&src[4..5]);
        let error = Error::new(src, token, String::from(""), 1);
        let back: &str = error.back();
        assert_eq!(back, &src[..4]);

        let token = Token::Eof(&src[5..5]);
        let error = Error::new(src, token, String::from(""), 1);
        let back: &str = error.back();
        assert_eq!(back, &src[..5]);
    }

    #[test]
    fn back_lf() {
        let src = "1 * 2\n3 + 4\n5 * 6";

        let token = Token::Number(&src[6..7]);
        assert_eq!(token, Token::Number("3"));
        let error = Error::new(src, token, String::from(""), 2);
        let back: &str = error.back();
        assert_eq!(back, &src[6..6]);

        let token = Token::Plus(&src[8..9]);
        assert_eq!(token, Token::Plus("+"));
        let error = Error::new(src, token, String::from(""), 2);
        let back: &str = error.back();
        assert_eq!(back, &src[6..8]);

        let token = Token::Number(&src[10..11]);
        assert_eq!(token, Token::Number("4"));
        let error = Error::new(src, token, String::from(""), 2);
        let back: &str = error.back();
        assert_eq!(back, &src[6..10]);

        let token = Token::Eof(&src[11..11]);
        assert_eq!(token, Token::Eof(""));
        let error = Error::new(src, token, String::from(""), 2);
        let back: &str = error.back();
        assert_eq!(back, &src[6..11]);
    }

    #[test]
    fn forth_no_lf() {
        let src = "1 + 2";

        let token = Token::Number(&src[0..1]);
        let error = Error::new(src, token, String::from(""), 1);
        let forth: &str = error.forth();
        assert_eq!(forth, &src[1..]);

        let token = Token::Plus(&src[2..3]);
        let error = Error::new(src, token, String::from(""), 1);
        let forth: &str = error.forth();
        assert_eq!(forth, &src[3..]);

        let token = Token::Number(&src[4..5]);
        let error = Error::new(src, token, String::from(""), 1);
        let forth: &str = error.forth();
        assert_eq!(forth, &src[5..]);
    }

    #[test]
    fn forth_lf() {
        let src = "1 * 2\n3 + 4\n5 * 6";

        let token = Token::Number(&src[6..7]);
        assert_eq!(token, Token::Number("3"));
        let error = Error::new(src, token, String::from(""), 2);
        let forth: &str = error.forth();
        assert_eq!(forth, &src[7..11]);

        let token = Token::Plus(&src[8..9]);
        assert_eq!(token, Token::Plus("+"));
        let error = Error::new(src, token, String::from(""), 2);
        let forth: &str = error.forth();
        assert_eq!(forth, &src[9..11]);

        let token = Token::Number(&src[10..11]);
        assert_eq!(token, Token::Number("4"));
        let error = Error::new(src, token, String::from(""), 2);
        let forth: &str = error.forth();
        assert_eq!(forth, &src[11..11]);
    }
}
