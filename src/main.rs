use token::{TokenType, Token};
use tokenizer::Tokenizer;

mod token;
mod tokenizer;

fn main() {
    let string = String::from("1 жопа + 1");
    let mut tokenizer = Tokenizer::new(&string);

    loop {
        let token: Token = tokenizer.scan_token();
        println!("{} {}", token._type, token.lexeme);
        if token._type == TokenType::EOF { break; }
    }
}
