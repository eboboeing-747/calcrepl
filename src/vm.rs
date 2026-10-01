use std::collections::HashMap;

pub type Num = f64;

#[derive(Debug, PartialEq)]
pub enum Code<'a> {
    Constant(Num),
    Variable(&'a str),
    Pop,

    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,

    Info,
    Exit,
}

pub struct VM {
    vars: HashMap<String, Num>,
    stack: Vec<Num>,
}

impl VM {
    pub fn new() -> Self {
        return VM {
            vars: HashMap::new(),
            stack: Vec::new(),
        };
    }

    // fn reset(&mut self) {
    //     self.code.clear();
    //     self.stack.clear();
    // }

    fn add_var(&mut self, name: String) {
        let value = self.pop();
        if self.vars.contains_key(&name)
            { panic!("attemt to redefine '{name}'"); }
        self.vars.insert(name, value);
    }

    fn pop(&mut self) -> Num {
        return self.stack.pop()
            .unwrap_or_else(|| panic!("attemt to pop from an empty stack"));
    }

    pub fn run(&mut self, code: &Vec<Code>) {
        for code in code {
            println!("{:?}", code);
            match code {
                Code::Constant(i) => self.stack.push(*i),
                Code::Variable(name) => self.add_var(name.to_string()),
                Code::Pop => { self.pop(); }
                Code::Add => {
                    let right = self.pop();
                    let left = self.pop();
                    self.stack.push(left + right);
                }
                Code::Subtract => {
                    let right = self.pop();
                    let left = self.pop();
                    self.stack.push(left - right);
                }
                Code::Multiply => {
                    let right = self.pop();
                    let left = self.pop();
                    self.stack.push(left * right);
                }
                Code::Divide => {
                    let right = self.pop();
                    let left = self.pop();
                    self.stack.push(left / right);
                }
                Code::Negate => {
                    let operand = self.pop();
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
        let mut vm = VM::new();
        vm.run(&code);
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
        let mut vm = VM::new();
        vm.run(&code);
        assert_eq!(vm.stack, vec![576.0]);
    }

    #[test]
    fn negate() {
        let code = vec![
            Code::Constant(1.0),
            Code::Negate,
        ];
        let mut vm = VM::new();
        vm.run(&code);
        assert_eq!(vm.stack, vec![-1.0]);
    }

    #[test]
    fn variable() {
        let mut vars = HashMap::new();
        vars.insert(String::from("name"), 1.0);
        let code = vec![
            Code::Constant(1.0),
            Code::Variable("name"),
        ];
        let mut vm = VM::new();
        vm.run(&code);
        assert_eq!(vm.vars, vars);
    }
}
