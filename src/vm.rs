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
    code: Vec<Code<'a>>,
    stack: Vec<Num>,
}

impl<'a> VM<'a> {
    pub fn new(code: Vec<Code<'a>>) -> Self {
        return VM {
            code: code,
            stack: Vec::new(),
        };
    }

    fn reset(&mut self) {
        self.code.clear();
        self.stack.clear();
    }

    pub fn run(&mut self) -> Num {
        println!();
        for code in &self.code {
            println!("{:?}", code);
            match code {
                Code::Constant(i) => self.stack.push(*i),
                Code::Variable(v) => todo!(),
                Code::Add => {
                    let right = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
                    let left = self.stack.pop()
                        .unwrap_or_else(|| panic!("stack is empty"));
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
                Code::Info => todo!(),
                Code::Exit => std::process::exit(0),
            }
        }

        return self.stack.pop().unwrap_or_else(|| panic!("runtime error"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let mut vm = VM::new(vec![
            Code::Constant(1.0),
            Code::Constant(1.0),
            Code::Add,
        ]);
        assert_eq!(vm.run(), 2.0);
    }

    #[test]
    fn expression() {
        let mut vm = VM::new(vec![
            Code::Constant(1920.0),
            Code::Constant(1920.0),
            Code::Constant(0.4),
            Code::Multiply,
            Code::Subtract,
            Code::Constant(2.0),
            Code::Divide,
        ]);
        assert_eq!(vm.run(), 576.0);
    }

    #[test]
    fn negate() {
        let mut vm = VM::new(vec![
            Code::Constant(1.0),
            Code::Negate,
        ]);
        assert_eq!(vm.run(), -1.0);
    }

    #[test]
    fn exit() {

    }
}
