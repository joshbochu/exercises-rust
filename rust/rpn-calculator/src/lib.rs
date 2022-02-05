#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut ret: Option<i32> = None;
    for i in inputs.iter() {
        match i {
            CalculatorInput::Value(x) => {
                vec.push(*x)
            },
            CalculatorInput::Subtract => {
                let mut num1 = vec.pop();
                let mut num2 = vec.pop();
                match (num1, num2) {
                    (Some(x), Some(y)) => {
                        vec.push(y-x);
                        Some(0)
                    },
                    _ => { return None; }
                };
            },
            CalculatorInput::Add => {
                let mut num1 = vec.pop();
                let mut num2 = vec.pop();
                match (num1, num2) {
                    (Some(x), Some(y)) => {
                        vec.push(y + x);
                        Some(0)
                    },
                    _ => { return None; }
                };
            },
            CalculatorInput::Multiply => {
                let mut num1 = vec.pop();
                let mut num2 = vec.pop();
                match (num1, num2) {
                    (Some(x), Some(y)) => {
                        vec.push(x*y);
                        Some(0)
                    },
                    _ => { return None; }
                };
            },
             CalculatorInput::Divide => {
                let mut num1 = vec.pop();
                let mut num2 = vec.pop();
                match (num1, num2) {
                    (Some(x), Some(y)) => {
                        vec.push(y / x);
                        Some(0)
                    },
                    _ => { return None; }
                };
            },
            _ => { return None; }
        }
    }
    if vec.len() == 1 { vec.pop() } else { None }
}