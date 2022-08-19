use crate::grammar::{Equation, EquationOneData, EquationTwoData, Number, Op};

pub fn equation(e: Equation) -> i32 {
    match e {
        Equation::EquationOne(EquationOneData(x0, x1, x2)) => {
            let Op(op) = x1;
            op(number(x0), equation(*x2))
        }
        Equation::EquationTwo(EquationTwoData(x0)) => number(x0),
    }
}

pub fn number(n: Number) -> i32 {
    n.0
}
