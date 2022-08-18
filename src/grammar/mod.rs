pub enum Equation {
    EquationOne(EquationOneData),
    EquationTwo(EquationTwoData),
}

pub struct EquationOneData {
    pub x0: Number,
    pub x1: Op,
    pub x2: Box<Equation>,
}

pub struct EquationTwoData {
    pub x0: Number,
}

pub struct Number {
    pub x0: i32,
}

pub struct Op {
    pub x0: Box<dyn std::ops::FnOnce(i32, i32) -> i32>,
}
