
#[derive(Debug)]

pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

//
// ```
// (4 + 8) / (7 - 5)
// ```

// can be written as

// ```
// 4 8 + 7 5 - /
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    // if input length is 0 return None
    if inputs.len() == 0 {
        return None;
    }
    //initialize stack vector
    let mut stack: Vec<i32> = Vec::new();

    // input in inputs match the input if it is a number and then push that number on the stack vector 
    for input in inputs {
        match input {
            // if input is a number
            CalculatorInput::Value(number) => stack.push(*number),
            // otherwise if stack length is less than two return None,
            // initialize variables b and be the last operators
            _=> {
                if stack.len() < 2 {
                    return None;
                }

                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                // match input with the operator and take the action on stack

                match input {
                    CalculatorInput::Add => stack.push(a + b),
                    CalculatorInput::Multiply => stack.push(a * b),
                    CalculatorInput::Subtract => stack.push(a - b),
                    CalculatorInput::Divide => stack.push(a / b),
                    _=> return None
                }
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }
    
    // return the stack item
    return stack.pop()

    
}

