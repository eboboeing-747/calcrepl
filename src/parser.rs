use core::panic;
use std::fmt;
use crate::tokenizer::Tokenizer;
use crate::token::{ Token, TokenType };

#[derive(Clone, Copy)]
enum BindingPower {
    Term = 1,
    Factor = 3,
}

pub enum S<'a> {
    Atom(Token<'a>),
    Cons(Token<'a>, Vec<S<'a>>),
}

impl fmt::Display for S<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i.lexeme),
            S::Cons(head, rest) => {
                write!(f, "({}", head.lexeme)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current: Token<'a>,
    previous: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let token = tokenizer.scan_token();
        return Parser {
            tokenizer: tokenizer,
            current: token,
            previous: Token::new_eof(),
        };
    }

    fn next(&mut self) -> Token<'a> {
        self.previous = self.current;
        self.current = self.tokenizer.scan_token();
        return self.previous;
    }

    fn peek(&mut self) -> Token<'a> { 
        return self.current;
    }

    pub fn parse(&mut self) -> S<'a> {
        return self.expr_bp(0);
    }

    fn expr_bp(&mut self, min_bp: u8) -> S<'a> {
        let token = self.next();
        let mut lhs = match token._type {
            TokenType::Number => S::Atom(token),
            t => panic!("expected number, got {} instead", t),
        };

        loop {
            let op_token = self.peek();
            let op = match op_token._type {
                TokenType::Eof => break,
                TokenType::Number =>
                    panic!("expected operator, got {} instead", token._type),
                TokenType::Error =>
                    panic!("expected operator, got {} instead", token._type),
                t => t,
            };
            let (l_bp, r_bp) = Self::infix_binding_power(op);
            if l_bp < min_bp { break; }
            self.next();
            let rhs = self.expr_bp(r_bp);

            lhs = S::Cons(op_token, vec![lhs, rhs]);
        }

        return lhs;
    }

    fn infix_binding_power(op: TokenType) -> (u8, u8) {
        let bp = match op {
            TokenType::Plus => BindingPower::Term,
            TokenType::Minus => BindingPower::Term,
            TokenType::Star => BindingPower::Factor,
            TokenType::Slash => BindingPower::Factor,
            t => panic!("unknown operator '{}'", t),
        };

        return (bp as u8, bp as u8 + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk() {
        let mut parser = Parser::new("1 + 2 * 3");
        assert_eq!(parser.peek(), Token::new(TokenType::Number, "1"));
        assert_eq!(parser.next(), Token::new(TokenType::Number, "1"));
        assert_eq!(parser.peek(), Token::new(TokenType::Plus, "+"));
        assert_eq!(parser.next(), Token::new(TokenType::Plus, "+"));
        assert_eq!(parser.peek(), Token::new(TokenType::Number, "2"));
        assert_eq!(parser.next(), Token::new(TokenType::Number, "2"));
        assert_eq!(parser.peek(), Token::new(TokenType::Star, "*"));
        assert_eq!(parser.next(), Token::new(TokenType::Star, "*"));
        assert_eq!(parser.peek(), Token::new(TokenType::Number, "3"));
        assert_eq!(parser.next(), Token::new(TokenType::Number, "3"));
        assert_eq!(parser.peek(), Token::new(TokenType::Eof, ""));
        assert_eq!(parser.next(), Token::new(TokenType::Eof, ""));
    }

    #[test]
    fn single_number() {
        let mut parser = Parser::new("1");
        let s = parser.parse();
        assert_eq!(s.to_string(), "1");
    }

    #[test]
    fn simple_expression() {
        let mut parser = Parser::new("1 + 1");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(+ 1 1)");
    }

    #[test]
    fn expression_bp_1() {
        let mut parser = Parser::new("1 + 2 * 3");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))")
    }

    #[test]
    fn expression_bp_2() {
        let mut parser = Parser::new("1 * 2 + 3");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(+ (* 1 2) 3)")
    }

    #[test]
    fn complicated_expression_1() {
        let mut parser = Parser::new("1 + 2 * 3 * 4 + 5");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(+ (+ 1 (* (* 2 3) 4)) 5)");
    }

    #[test]
    fn complicated_expression_2() {
        let mut parser = Parser::new("1 * 2 * 3 + 4 * 5");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(+ (* (* 1 2) 3) (* 4 5))");
    }
}
