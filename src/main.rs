use parser::Parser;
use std::io::{ Write, stdin };
use vm::{ Code, VM };

mod token;
mod tokenizer;
mod parser;
mod vm;

fn main() {
    let mut vm = VM::new();
    loop {
        let mut line = String::new();
        print!("\n> ");
        let _ = std::io::stdout().flush();
        stdin().read_line(&mut line).expect("failed to read line");
        let mut code: Vec<Code> = Vec::new();

        let mut parser = Parser::new(&line, &mut code);
        parser.parse();

        vm.run(&code);
    }
}
