use std::fmt::Display;

use crate::token::Token;

pub struct Error<'a> {
    source_file: &'a str,
    token: Token<'a>,
    message: String,
}

impl<'a> Error<'a> {
    pub fn new(source_file: &'a str, token: Token<'a>, message: String) -> Self {
        return Error { source_file, token, message };
    }

    fn back(&self) -> &str {
        todo!();
    }

    fn forth(&self) -> &str {
        todo!();
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn back_no_lf() {
        let src = "1 + 2";

        let token = Token::Number(&src[0..1]);
        let error = Error::new(src, token, String::from(""));
        let back: &str = error.back();
        assert_eq!(back, &src[..0]);

        let token = Token::Plus(&src[2..3]);
        let error = Error::new(src, token, String::from(""));
        let back: &str = error.back();
        assert_eq!(back, &src[..2]);

        let token = Token::Number(&src[4..5]);
        let error = Error::new(src, token, String::from(""));
        let back: &str = error.back();
        assert_eq!(back, &src[..4]);
    }

    #[test]
    fn back_lf() {
        let src = "1 + 2\n3 + 4\n5 + 6";

        let token = Token::Number(&src[7..8]);
        let error = Error::new(src, token, String::from(""));
        let back: &str = error.back();
        assert_eq!(back, &src[7..7]);

        let token = Token::Plus(&src[9..10]);
        let error = Error::new(src, token, String::from(""));
        let back: &str = error.back();
        assert_eq!(back, &src[7..9]);

        let token = Token::Number(&src[11..12]);
        let error = Error::new(src, token, String::from(""));
        let back: &str = error.back();
        assert_eq!(back, &src[7..11]);
    }

    #[test]
    fn forth_no_lf() {
        let src = "1 + 2";

        let token = Token::Number(&src[0..1]);
        let error = Error::new(src, token, String::from(""));
        let forth: &str = error.forth();
        assert_eq!(forth, &src[1..]);

        let token = Token::Plus(&src[2..3]);
        let error = Error::new(src, token, String::from(""));
        let forth: &str = error.forth();
        assert_eq!(forth, &src[3..]);

        let token = Token::Number(&src[4..5]);
        let error = Error::new(src, token, String::from(""));
        let forth: &str = error.forth();
        assert_eq!(forth, &src[5..]);
    }

    #[test]
    fn forth_lf() {
        let src = "1 + 2\n3 + 4\n5 + 6";

        let token = Token::Number(&src[7..8]);
        let error = Error::new(src, token, String::from(""));
        let forth: &str = error.forth();
        assert_eq!(forth, &src[8..12]);

        let token = Token::Plus(&src[9..10]);
        let error = Error::new(src, token, String::from(""));
        let forth: &str = error.forth();
        assert_eq!(forth, &src[10..12]);

        let token = Token::Number(&src[11..12]);
        let error = Error::new(src, token, String::from(""));
        let forth: &str = error.forth();
        assert_eq!(forth, &src[12..12]);
    }
}
