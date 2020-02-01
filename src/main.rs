use std::env;

fn main() {
    handle_arguments()
}

enum Operator {
    subtraction,
    addition,
    multiplication,
    division,
    remainder,
    unknown,
}

impl Operator {
    fn new(parse_str: &str) -> Operator {
        let result: Operator = match parse_str {
            "+" => Operator::addition,
            "-" => Operator::subtraction,
            "x" => Operator::multiplication,
            "/" => Operator::division,
            "%" => Operator::remainder,
            _ => Operator::unknown,
        };

        return result;
    }
}

fn handle_arguments() {
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];
    let lhs = &args[1];
    let operator = &args[2];
    let rhs = &args[3];

    let error_message: String = "Incorrect Input".to_owned();
    let left_number: i32 = lhs.parse().expect(&error_message);
    let right_number: i32 = rhs.parse().expect(&error_message);
    let inputs: (i32, i32) = (left_number, right_number);

    let operator_string: String = operator.parse().expect(&error_message);

    let operator_obj: Operator = Operator::new(operator_string.as_str());
    let maths: Mathematics = Mathematics::new();
    let result = maths.equation(inputs.0, inputs.1, operator_obj);

    println!(
        "Output: {} {} {} = {}",
        inputs.0, operator_string, inputs.1, result
    );
}

struct Mathematics {}

impl Mathematics {
    fn new() -> Mathematics {
        Mathematics {}
    }

    fn addition(&self, lhs: i32, rhs: i32) -> i32 {
        let result = lhs + rhs;
        return result;
    }

    fn subtraction(&self, lhs: i32, rhs: i32) -> i32 {
        let result = lhs - rhs;
        return result;
    }

    fn multipliction(&self, lhs: i32, rhs: i32) -> i32 {
        let result = lhs * rhs;
        return result;
    }

    fn division(&self, lhs: i32, rhs: i32) -> i32 {
        let result = lhs / rhs;
        return result;
    }

    fn remainder(&self, lhs: i32, rhs: i32) -> i32 {
        let result = lhs % rhs;
        return result;
    }

    pub fn equation(&self, lhs: i32, rhs: i32, operator: Operator) -> i32 {
        let result: i32 = match operator {
            Operator::addition => self.addition(lhs, rhs),
            Operator::subtraction => self.subtraction(lhs, rhs),
            Operator::division => self.division(lhs, rhs),
            Operator::remainder => self.remainder(lhs, rhs),
            Operator::multiplication => self.multipliction(lhs, rhs),
            Operator::unknown => 0,
        };

        return result;
    }
}
