mod mathematics;
mod operator;

use mathematics::Mathematics;
use operator::Symbol;
use std::env;

fn main() {
    handle_arguments()
}

fn handle_arguments() {
    let args: Vec<String> = env::args().collect();

    let lhs = &args[1];
    let operator = &args[2];
    let rhs = &args[3];

    let error_message: String = "Incorrect Input".to_owned();
    let left_number: i32 = lhs.parse().expect(&error_message);
    let right_number: i32 = rhs.parse().expect(&error_message);
    let inputs: (i32, i32) = (left_number, right_number);

    let operator_string: String = operator.parse().expect(&error_message);

    let operator_obj: Symbol = Symbol::new(operator_string.as_str()).unwrap();
    let maths: Mathematics = Mathematics::new();
    let result = maths.equation(inputs.0, inputs.1, operator_obj);

    println!(
        "Output: {} {} {} = {}",
        inputs.0, operator_string, inputs.1, result
    );
}

#[test]
fn testAddition() {
    let maths = Mathematics::new();
    let lhs = 5;
    let rhs = 10;

    maths.equation(lhs, rhs, Symbol::Addition);
}
