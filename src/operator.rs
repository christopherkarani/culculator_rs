pub struct Equation {
    lhs: i32,
    rhs: i32,
    operator: Symbol,
}

pub enum Symbol {
    Subtraction,
    Addition,
    Multiplication,
    Division,
    Remainder,
}

impl Symbol {
    pub fn new(parse_str: &str) -> Option<Symbol> {
        let result: Option<Symbol> = match parse_str {
            "+" => Some(Symbol::Addition),
            "-" => Some(Symbol::Subtraction),
            "x" => Some(Symbol::Multiplication),
            "/" => Some(Symbol::Division),
            "%" => Some(Symbol::Remainder),
            _ => None,
        };
        return result;
    }
}
