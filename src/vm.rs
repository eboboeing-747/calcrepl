type Num = f64;

enum Code {
    Constant(Num),
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct VM {
    code: Vec<Code>,
    stack: Vec<Num>,
}

impl VM {
    fn new(code: Vec<Code>) -> Self {
        return VM {
            code: code,
            stack: Vec::new(),
        };
    }

    fn run(&mut self) {
        for code in &self.code {
            match code {
                Code::Constant(i) => self.stack.push(*i),
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
            }
        }
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
        vm.run();
        assert_eq!(vm.stack, vec![2.0]);
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
        vm.run();
        assert_eq!(vm.stack, vec![576.0]);
    }
}
