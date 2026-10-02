use std::collections::HashMap;

pub type Num = f64;

#[derive(Debug, PartialEq)]
pub enum Code<'a> {
    Constant(Num),
    NewVariable(&'a str),
    SetVariable(&'a str),
    GetVariable(&'a str),
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
        let value = self.peek();
        if self.vars.contains_key(&name)
            { panic!("attemt to redefine '{name}'"); }
        self.vars.insert(name, value);
    }

    fn set_var(&mut self, name: &str) {
        let new_value = self.peek();
        let var = self.vars.get_mut(name);
        match var {
            Some(value) => *value = new_value,
            None => panic!("'{name}' is undefined"),
        }
    }

    fn peek(&self) -> Num {
        return self.stack[self.stack.len() - 1];
    }

    fn pop(&mut self) -> Num {
        return self.stack.pop()
            .unwrap_or_else(|| panic!("attemt to pop from an empty stack"));
    }

    pub fn run(&mut self, code: &Vec<Code>) {
        for code in code {
            println!("{:?}", code);
            match *code {
                Code::Constant(i) => self.stack.push(i),
                Code::NewVariable(name) => self.add_var(name.to_string()),
                Code::SetVariable(name) => self.set_var(name),
                Code::GetVariable(name) => {
                    let var = self.vars.get(name);
                    match var {
                        Some(value) => self.stack.push(*value),
                        None => panic!("'{name}' is undefined"),
                    }
                }
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
    fn new_variable() {
        let vars = HashMap::from([(String::from("name"), 1.0)]);
        let code = vec![
            Code::Constant(1.0),
            Code::NewVariable("name"),
        ];
        let mut vm = VM::new();
        vm.run(&code);
        assert_eq!(vm.vars, vars);
    }

    #[test]
    #[should_panic(expected = "attemt to redefine 'name'")]
    fn redefine_variable() {
        let code = vec![
            Code::Constant(1.0),
            Code::NewVariable("name"),
            Code::Constant(2.0),
            Code::NewVariable("name"),
        ];
        let mut vm = VM::new();
        vm.run(&code);
    }

    #[test]
    fn set_variable() {
        let mut vm = VM::new();
        vm.vars.insert(String::from("name"), 10.0);
        let code = vec![Code::Constant(1.0), Code::SetVariable("name")];
        vm.run(&code);
        assert_eq!(*vm.vars.get("name").unwrap(), 1.0);
    }

    #[test]
    #[should_panic(expected = "'name' is undefined")]
    fn set_undefined() {
        let mut vm = VM::new();
        vm.vars.insert(String::from("foo"), 10.0);
        let code = vec![Code::Constant(1.0), Code::SetVariable("name")];
        vm.run(&code);
    }

    #[test]
    fn get_variable() {
        let mut vm = VM::new();
        vm.vars.insert(String::from("name"), 10.0);
        let code = vec![Code::GetVariable("name")];
        vm.run(&code);
        assert_eq!(vm.stack, vec![10.0]);
    }

    #[test]
    #[should_panic(expected = "'name' is undefined")]
    fn get_undefined() {
        let mut vm = VM::new();
        let code = vec![Code::GetVariable("name")];
        vm.run(&code);
    }
}
