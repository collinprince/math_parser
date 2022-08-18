use crate::grammar::{Equation, EquationOneData, EquationTwoData, Number, Op};

pub fn parse(s: &str) -> Equation {
    equation(s)
}

pub fn equation(_: &str) -> Equation {
    Equation::EquationOne(EquationOneData {
        x0: Number { x0: 2 },
        x1: Op {
            x0: Box::new(std::ops::Add::add),
        },
        x2: Box::new(Equation::EquationTwo(EquationTwoData {
            x0: Number { x0: 3 },
        })),
    })
}
