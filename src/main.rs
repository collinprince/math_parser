use math_parser::interpreter::equation;
use math_parser::parser::parse;

fn main() {
    println!("{}", equation(parse("")));
}
