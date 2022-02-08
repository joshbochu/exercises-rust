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
    for i in inputs {
        match i {
            CalculatorInput::Value(v) => vec.push(*v),
            _ => match vec.len() {
                0 | 1 => return None,
                _ => {
                    let a = vec.pop().unwrap();
                    let b = vec.pop().unwrap();
                    match i {
                        CalculatorInput::Add => vec.push(b + a),
                        CalculatorInput::Subtract => vec.push(b - a),
                        CalculatorInput::Multiply => vec.push(b * a),
                        CalculatorInput::Divide => vec.push(b / a),
                        _ => return None
                    }
                }
            }
        }
    }
    if vec.len() == 1 { vec.pop() } else { None }
}