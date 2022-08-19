use std::any::Any;

pub trait GrammarElement {
    fn as_any(&self) -> &dyn Any;
}

pub enum Equation {
    EquationOne(EquationOneData),
    EquationTwo(EquationTwoData),
}
impl GrammarElement for Equation {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EquationOneData(pub Number, pub Op, pub Box<Equation>);
pub struct EquationTwoData(pub Number);

pub struct Number(pub i32);
impl GrammarElement for Number {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Op(pub Box<dyn std::ops::FnOnce(i32, i32) -> i32>);
impl GrammarElement for Op {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
