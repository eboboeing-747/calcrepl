use crate::tokenizer::Tokenizer;
use crate::token::Token;
use crate::vm::{ Num, Code };

#[derive(Clone, Copy)]
enum BindingPower {
    Assignment = 1,
    Term = 3,
    Factor = 5,
    Negate = 7,
}

pub struct Parser<'src, 'a> {
    tokenizer: Tokenizer<'src>,
    code: &'a mut Vec<Code<'src>>,
    current: Token<'src>,
    previous: Token<'src>,
}

impl<'src, 'a> Parser<'src, 'a> {
    pub fn new(input: &'src str, code: &'a mut Vec<Code<'src>>) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let token = tokenizer.scan_token();
        return Parser {
            tokenizer: tokenizer,
            code: code,
            current: token,
            previous: Token::Eof,
        };
    }

    fn next(&mut self) -> Token<'src> {
        self.previous = self.current;
        self.current = self.tokenizer.scan_token();
        return self.previous;
    }

    fn peek(&mut self) -> Token<'src> { 
        return self.current;
    }

    fn expect(&mut self, expected: Token, message: &'static str) -> Token<'src> {
        let token = self.next();
        if std::mem::discriminant(&token) != std::mem::discriminant(&expected)
            { panic!("{message}"); }
        return token;
    }

    fn _match(&mut self, token: Token) -> bool {
        if std::mem::discriminant(&self.peek()) ==
            std::mem::discriminant(&token)
        {
            self.next();
            return true;
        }
        return false;
    }

    pub fn parse(&mut self) {
        while self.peek() != Token::Eof {
            self.statement();
        }
    }

    fn let_stmt(&mut self) {
        self.next();
        let name = self.expect(Token::Identifier(""),
            "expect identifier after 'let'");
        self.expect(Token::Equal(""), "expect '=' after 'let <name>'");
        self.expr_bp(0);
        self.expect(Token::Semicolon(""),
            "expect ';' after initializer expression");
        self.emit(Code::NewVariable(match name {
            Token::Identifier(lexeme) => lexeme,
            _ => panic!("error"),
        }));
    }

    fn info_stmt(&mut self) {
        self.next();
        self.emit(Code::Info);
        self.expect(Token::Semicolon(""), "expect ';' after 'info'");
    }

    fn exit_stmt(&mut self) {
        self.next();
        self.emit(Code::Exit);
        self.expect(Token::Semicolon(""), "expect ';' after 'exit'");
    }

    fn expr_stmt(&mut self) {
        self.expr_bp(0);
        self.expect(Token::Semicolon(""), "expect ';' after expression");
        self.emit(Code::Pop);
    }

    fn statement(&mut self) {
        match self.peek() {
            Token::Identifier("let") => self.let_stmt(),
            Token::Identifier("info") => self.info_stmt(),
            Token::Identifier("exit") => self.exit_stmt(),
            _ => self.expr_stmt(),
        }
    }

    fn number(&mut self, lexeme: &str) {
        let number: Num = lexeme.parse().expect("failed to parse");
        self.emit(Code::Constant(number));
    }

    fn variable(&mut self, lexeme: &'src str, can_assign: bool) {
        if can_assign && self._match(Token::Equal("")) {
            self.expr_bp(BindingPower::Assignment as u8);
            self.emit(Code::SetVariable(lexeme));
        } else {
            self.emit(Code::GetVariable(lexeme));
        }
    }

    fn emit(&mut self, code: Code<'src>) {
        self.code.push(code);
    }

    fn expr_bp(&mut self, min_bp: u8) {
        let can_assign = min_bp <= BindingPower::Assignment as u8;
        let token = self.next();
        let _lhs = match token {
            Token::Number(lexeme) => self.number(lexeme),
            Token::Identifier(lexeme) => self.variable(lexeme, can_assign),
            Token::Minus(_lexeme) => {
                let ((), r_bp) = Self::prefix_binding_power(token);
                self.expr_bp(r_bp);
                self.emit(Code::Negate);
            }
            Token::LeftParen(_lexeme) => {
                self.expr_bp(0);
                assert_eq!(self.next(), Token::RightParen(")"));
            }
            t => panic!("expected Token::Number, got {} instead", t),
        };

        loop {
            let op = match self.peek() {
                Token::Eof | Token::RightParen(_) |
                    Token::Semicolon(_) | Token::Equal(_) => break,
                Token::Number(_lexeme) =>
                    panic!("expected Operator, got {token} instead"),
                Token::Error(_lexeme) =>
                    panic!("expected Operator, got {token} instead"),
                t => t,
            };

            let (l_bp, r_bp) = Self::infix_binding_power(op);
            if l_bp < min_bp { break; }
            self.next();
            let _rhs = self.expr_bp(r_bp);

            match op {
                Token::Plus(_lexeme) => self.emit(Code::Add),
                Token::Minus(_lexeme) => self.emit(Code::Subtract),
                Token::Star(_lexeme) => self.emit(Code::Multiply),
                Token::Slash(_lexeme) => self.emit(Code::Divide),
                _ => panic!("parsing error"),
            }
        }

        if can_assign && self._match(Token::Equal("")) {
            panic!("invalid assignment target");
        }
    }

    fn prefix_binding_power(op: Token) -> ((), u8) { 
        match op {
            Token::Minus(_lexeme) => ((), BindingPower::Negate as u8),
            _ => panic!("bad op: {:?}", op),
        }
    }

    fn infix_binding_power(op: Token) -> (u8, u8) {
        let bp = match op {
            Token::Plus(_) => BindingPower::Term,
            Token::Minus(_) => BindingPower::Term,
            Token::Star(_) => BindingPower::Factor,
            Token::Slash(_) => BindingPower::Factor,
            Token::Equal(_) => return (BindingPower::Assignment as u8 + 1,
                BindingPower::Assignment as u8),
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
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 + 2 * 3", &mut code);
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
    #[should_panic(expected = "expect identifier")]
    fn expect_1() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("+", &mut code);
        parser.expect(Token::Identifier("anything"), "expect identifier");
    }

    #[test]
    fn expect_2() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("+", &mut code);
        parser.expect(Token::Plus("anything"), "expect operator");
    }

    #[test]
    fn _match() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1234 + name", &mut code);
        assert_eq!(parser._match(Token::Number("anything")), true);
        assert_eq!(parser.peek(), Token::Plus("+"));
        assert_eq!(parser._match(Token::Slash("anything")), false);
        assert_eq!(parser.peek(), Token::Plus("+"));
        assert_eq!(parser._match(Token::Plus("anything")), true);
        assert_eq!(parser._match(Token::Identifier("anything")), true);
        assert_eq!(parser._match(Token::Eof), true);
    }

    #[test]
    fn single_number() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![Code::Constant(1.0)]);
    }

    #[test]
    fn infix_expression() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 + 2", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Add,
        ]);
    }

    #[test]
    fn infix_expression_bp_1() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 + 2 * 3", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Constant(3.0),
            Code::Multiply,
            Code::Add,
        ])
    }

    #[test]
    fn infix_expression_bp_2() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 * 2 + 3", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Multiply,
            Code::Constant(3.0),
            Code::Add,
        ])
    }

    #[test]
    fn complicated_infix_expression_1() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 + 2 * 3 * 4 + 5", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Constant(3.0),
            Code::Multiply,
            Code::Constant(4.0),
            Code::Multiply,
            Code::Add,
            Code::Constant(5.0),
            Code::Add,
        ]);
    }

    #[test]
    fn complicated_infix_expression_2() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 * 2 * 3 + 4 * 5", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Multiply,
            Code::Constant(3.0),
            Code::Multiply,
            Code::Constant(4.0),
            Code::Constant(5.0),
            Code::Multiply,
            Code::Add,
        ]);
    }

    #[test]
    fn prefix_expr_1() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("-1", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Negate,
        ]);
    }

    #[test]
    fn prefix_expr_2() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("--1", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Negate,
            Code::Negate,
        ]);
    }

    #[test]
    fn infix_prefix_expression_1() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 + -2 - -3", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Negate,
            Code::Add,
            Code::Constant(3.0),
            Code::Negate,
            Code::Subtract,
        ]);
    }

    #[test]
    fn infix_prefix_expression_2() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 + -2 * 3 - -4", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Negate,
            Code::Constant(3.0),
            Code::Multiply,
            Code::Add,
            Code::Constant(4.0),
            Code::Negate,
            Code::Subtract,
        ]);
    }

    #[test]
    fn infix_prefix_expression_3() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("-1 + 2 * -3 - 4", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Negate,
            Code::Constant(2.0),
            Code::Constant(3.0),
            Code::Negate,
            Code::Multiply,
            Code::Add,
            Code::Constant(4.0),
            Code::Subtract,
        ]);
    }

    #[test]
    fn variable() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("foo", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![Code::GetVariable("foo")]);
    }

    #[test]
    fn complex_with_variables() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("-foo + 2 * -bar - 4", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::GetVariable("foo"),
            Code::Negate,
            Code::Constant(2.0),
            Code::GetVariable("bar"),
            Code::Negate,
            Code::Multiply,
            Code::Add,
            Code::Constant(4.0),
            Code::Subtract,
        ]);
    }

    #[test]
    fn paren_expression_1() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("(1)", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
        ]);
    }

    #[test]
    fn paren_expression_2() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("(((((1)))))", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
        ]);
    }

    #[test]
    fn paren_expression_3() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("(1 + 2) * 3", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Add,
            Code::Constant(3.0),
            Code::Multiply,
        ]);
    }

    #[test]
    fn paren_expression_4() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 / 2 * (3 - 4) + 5", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Divide,
            Code::Constant(3.0),
            Code::Constant(4.0),
            Code::Subtract,
            Code::Multiply,
            Code::Constant(5.0),
            Code::Add,
        ]);
    }

    #[test]
    fn assignment_1() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("foo = 5", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(5.0),
            Code::SetVariable("foo"),
        ]);
    }

    #[test]
    fn assignment_2() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("foo = 1 - 2 / 3", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Constant(3.0),
            Code::Divide,
            Code::Subtract,
            Code::SetVariable("foo"),
        ]);
    }

    #[test]
    fn assignment_3() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("foo = 1 - (bar = 2 / 3)", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::Constant(2.0),
            Code::Constant(3.0),
            Code::Divide,
            Code::SetVariable("bar"),
            Code::Subtract,
            Code::SetVariable("foo"),
        ]);
    }

    #[test]
    fn assignment_4() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("foo = bar = baz = 1", &mut code);
        parser.expr_bp(0);
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::SetVariable("baz"),
            Code::SetVariable("bar"),
            Code::SetVariable("foo"),
        ]);
    }

    #[test]
    #[should_panic(expected = "invalid assignment target")]
    fn assignment_5() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("foo = 1 - bar = 2 / 3", &mut code);
        parser.expr_bp(0);
    }

    #[test]
    #[should_panic(expected = "invalid assignment target")]
    fn assignment_invalid() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("1 - foo = 2", &mut code);
        parser.expr_bp(0);
    }

    #[test]
    fn let_stmt() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("let name = 1;", &mut code);
        parser.parse();
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::NewVariable("name"),
        ]);
    }

    #[test]
    #[should_panic(expected = "expect '=' after 'let <name>'")]
    fn let_stmt_error_1() {
        let mut code: Vec<Code> = Vec::new();
        let mut parser = Parser::new("let name 1;", &mut code);
        parser.parse();
        assert_eq!(code, vec![
            Code::Constant(1.0),
            Code::NewVariable("name"),
        ]);
    }
}
