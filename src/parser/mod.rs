use crate::grammar::{Equation, EquationOneData, EquationTwoData, Number, Op};
use crate::tokenizer::Token;
pub fn parse<'a>(v: &[Token]) -> Result<EquationOutput, String> {
    equation(v)
}
pub struct EquationOutput<'a> {
    pub result: Box<Equation>,
    pub continuation: &'a [Token],
}
pub fn equation(v: &[Token]) -> Result<EquationOutput, String> {
    println!("{:?}", v);
    match v.len() {
        0 => Err(String::from("Invalid")),
        1 | 2 => {
            if let Token::Number(n) = v[0] {
                Ok(EquationOutput {
                    result: Box::new(Equation::EquationTwo(EquationTwoData(Number(n)))),
                    continuation: &v[1..],
                })
            } else {
                Err(String::from("Invalid"))
            }
        }
        _ => {
            let first = if let Token::Number(n) = v[0] {
                Ok(Number(n))
            } else {
                Err(String::from("Invalid"))
            };
            // first?;
            let second = {
                let op_val = op(&v[1]);
                // op_val?;
                println!("op: {:?}", v[1]);
                Ok(op_val.unwrap()) as Result<Op, String>
            };
            // second?;
            let third = equation(&v[2..]);
            // third?;
            let EquationOutput {
                result: third_equation,
                continuation,
            } = third.unwrap();
            Ok(EquationOutput {
                result: Box::new(Equation::EquationOne(EquationOneData(
                    first.unwrap(),
                    second.unwrap(),
                    third_equation,
                ))),
                continuation,
            })
        }
    }
}

pub fn op(t: &Token) -> Result<Op, String> {
    match t {
        Token::Op(o) => {
            if o.eq("+") {
                Ok(Op(Box::new(std::ops::Add::add)))
            } else if o.eq("-") {
                Ok(Op(Box::new(std::ops::Sub::sub)))
            } else if o.eq("*") {
                Ok(Op(Box::new(std::ops::Mul::mul)))
            } else if o.eq("/") {
                Ok(Op(Box::new(std::ops::Div::div)))
            } else {
                Err("Invalid operator token".to_string())
            }
        }
        _ => Err("Invalid operator token".to_string()),
    }
}
