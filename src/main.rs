use math_parser::interpreter::equation;
use math_parser::parser::parse;
use math_parser::tokenizer::tokenize;

fn main() {
    let input: &str = "1 + 2 * 9";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
    if let Ok(tokens) = tokens {
        let res = parse(&tokens).unwrap();
        println!("{}", equation(*res.result));
    } else {
        println!("Error in tokenization.");
    }
}
