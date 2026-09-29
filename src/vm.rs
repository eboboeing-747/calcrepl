use std::collections::HashMap;

pub type Num = f64;

#[derive(Debug, PartialEq)]
pub enum Code<'a> {
    Constant(Num),
    Variable(&'a str),

    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,

    Info,
    Exit,
}

pub struct VM<'a> {
    code: &'a Vec<Code<'a>>,
    vars: HashMap<&'a str, Num>,
    stack: Vec<Num>,
}

impl<'a> VM<'a> {
    pub fn new(code: &'a Vec<Code<'a>>) -> Self {
        return VM {
            code: code,
            vars: HashMap::new(),
            stack: Vec::new(),
        };
    }

    // fn reset(&mut self) {
    //     self.code.clear();
    //     self.stack.clear();
    // }

    fn pop(&mut self) -> Num {
        return self.stack.pop()
            .unwrap_or_else(|| panic!("attemt to pop from an empty stack"));
    }

    pub fn run(&mut self) {
        println!();
        for code in self.code {
            println!("{:?}", code);
            match code {
                Code::Constant(i) => self.stack.push(*i),
                Code::Variable(name) => {
                    let value = self.pop();
                    self.vars.insert(name, value);
                }
                Code::Add => {
                    let right = self.pop();
                    let left = self.pop();
                    self.stack.push(left + right);
                }
                Code::Subtract => {
                    let right = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
                    let left = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
                    self.stack.push(left - right);
                }
                Code::Multiply => {
                    let right = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
                    let left = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
                    self.stack.push(left * right);
                }
                Code::Divide => {
                    let right = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
                    let left = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
                    self.stack.push(left / right);
                }
                Code::Negate => {
                    let operand = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
                    self.stack.push(-operand);
                }
                Code::Info => {
                    for i in &self.vars {
                        println!("{} {}", i.0, i.1);
                    }
                }
                Code::Exit => std::process::exit(0),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let code = vec![
            Code::Constant(1.0),
            Code::Constant(1.0),
            Code::Add,
        ];
        let mut vm = VM::new(&code);
        vm.run();
        assert_eq!(vm.stack, vec![2.0]);
    }

    #[test]
    fn expression() {
        let code = vec![
            Code::Constant(1920.0),
            Code::Constant(1920.0),
            Code::Constant(0.4),
            Code::Multiply,
            Code::Subtract,
            Code::Constant(2.0),
            Code::Divide,
        ];
        let mut vm = VM::new(&code);
        vm.run();
        assert_eq!(vm.stack, vec![576.0]);
    }

    #[test]
    fn negate() {
        let code = vec![
            Code::Constant(1.0),
            Code::Negate,
        ];
        let mut vm = VM::new(&code);
        vm.run();
        assert_eq!(vm.stack, vec![-1.0]);
    }

    #[test]
    fn variable() {
        let mut vars = HashMap::new();
        vars.insert("name", 1.0);
        let code = vec![
            Code::Constant(1.0),
            Code::Variable("name"),
        ];
        let mut vm = VM::new(&code);
        vm.run();
        assert_eq!(vm.vars, vars);
    }
}
