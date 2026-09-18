use core::panic;
use std::fmt;
use crate::tokenizer::Tokenizer;
use crate::token::Token;

#[derive(Clone, Copy)]
enum BindingPower {
    Term = 1,
    Factor = 3,
    Negate = 5,
}

pub enum S<'a> {
    Atom(Token<'a>),
    Cons(Token<'a>, Vec<S<'a>>),
}

impl fmt::Display for S<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(token) => write!(f, "{}", token),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
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
            previous: Token::Eof,
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
        let mut lhs = match self.next() {
            number @ Token::Number(_lexeme) => S::Atom(number),
            op @ Token::Minus(_lexeme) => {
                let ((), r_bp) = Self::prefix_binding_power(op);
                let rhs = self.expr_bp(r_bp);
                S::Cons(op, vec![rhs])
            }
            t => panic!("expected Token::Number, got {} instead", t),
        };

        loop {
            let op = match self.peek() {
                Token::Eof => break,
                valid @ (Token::Plus(_) | Token::Minus(_) |
                    Token::Star(_) | Token::Slash(_)) => valid,
                _ => panic!("expected operator"),
            };

            let (l_bp, r_bp) = Self::infix_binding_power(op);
            if l_bp < min_bp { break; }
            self.next();
            let rhs = self.expr_bp(r_bp);

            lhs = S::Cons(op, vec![lhs, rhs]);
        }

        return lhs;
    }

    fn prefix_binding_power(op: Token) -> ((), u8) {
        return match op {
            Token::Minus(_lexeme) => ((), BindingPower::Negate as u8),
            t => panic!("unknown operator '{}'", t),
        }
    }

    fn infix_binding_power(op: Token) -> (u8, u8) {
        let bp = match op {
            Token::Plus(_lexeme) => BindingPower::Term,
            Token::Minus(_lexeme) => BindingPower::Term,
            Token::Star(_lexeme) => BindingPower::Factor,
            Token::Slash(_lexeme) => BindingPower::Factor,
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
        assert_eq!(parser.peek(), Token::Number("1"));
        assert_eq!(parser.next(), Token::Number("1"));
        assert_eq!(parser.peek(), Token::Plus("+"));
        assert_eq!(parser.next(), Token::Plus("+"));
        assert_eq!(parser.peek(), Token::Number("2"));
        assert_eq!(parser.next(), Token::Number("2"));
        assert_eq!(parser.peek(), Token::Star("*"));
        assert_eq!(parser.next(), Token::Star("*"));
        assert_eq!(parser.peek(), Token::Number("3"));
        assert_eq!(parser.next(), Token::Number("3"));
        assert_eq!(parser.peek(), Token::Eof);
        assert_eq!(parser.next(), Token::Eof);
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

    #[test]
    fn prefix_1() {
        let mut parser = Parser::new("-1");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(- 1)");
    }

    #[test]
    fn prefix_2() {
        let mut parser = Parser::new("--1");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(- (- 1))");
    }

    #[test]
    fn infix_prefix_1() {
        let mut parser = Parser::new("1 + -2 * 3 - -4");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(- (+ 1 (* (- 2) 3)) (- 4))");
    }

    #[test]
    fn infix_prefix_2() {
        let mut parser = Parser::new("-1 + 2 * -3 - 4");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(- (+ (- 1) (* 2 (- 3))) 4)");
    }

    #[test]
    fn infix_prefix_3() {
        let mut parser = Parser::new("-1 + -2 * -3 - -4");
        let s = parser.parse();
        assert_eq!(s.to_string(), "(- (+ (- 1) (* (- 2) (- 3))) (- 4))");
    }
}
