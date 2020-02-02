use crate::operator;
use operator::Symbol;

pub struct Mathematics {}

impl Mathematics {
    pub fn new() -> Mathematics {
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

    pub fn equation(&self, lhs: i32, rhs: i32, operator: Symbol) -> i32 {
        let result: i32 = match operator {
            Symbol::Addition => self.addition(lhs, rhs),
            Symbol::Subtraction => self.subtraction(lhs, rhs),
            Symbol::Division => self.division(lhs, rhs),
            Symbol::Remainder => self.remainder(lhs, rhs),
            Symbol::Multiplication => self.multipliction(lhs, rhs),
        };

        return result;
    }
}
