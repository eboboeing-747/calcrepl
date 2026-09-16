use parser::Parser;
use std::io::{Write, stdin};

mod token;
mod tokenizer;
mod parser;

fn main() {
    loop {
        let mut line = String::new();
        print!("> ");
        let _ = std::io::stdout().flush();
        stdin().read_line(&mut line).expect("failed to read line");
        let line: &str = line.trim();
        if line == "exit" { break; }
        let mut parser = Parser::new(&line);
        println!("{}", parser.expr())
    }
}
