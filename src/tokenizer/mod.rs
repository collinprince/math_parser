use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Op(String),
    Number(i32),
}

pub fn tokenize<'a>(s: &str) -> Result<Vec<Token>, &str> {
    s.split_whitespace()
        .map(|elem| {
            if is_op(elem) {
                Ok(Token::Op(elem.to_string()))
            } else if is_number(elem) {
                let num: i32 = elem.parse().unwrap();
                Ok(Token::Number(num))
            } else {
                Err("tokenization error")
            }
        })
        .collect()
}

pub fn is_op(s: &str) -> bool {
    s.eq("+") || s.eq("-") || s.eq("*") || s.eq("/")
}

pub fn is_number(s: &str) -> bool {
    let re = Regex::new("^[0-9]+$").unwrap();
    re.is_match(s)
}
