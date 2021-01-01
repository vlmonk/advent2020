use crate::lexer::{MathLexer, Token};
use std::fmt;

pub enum Expr {
    Num(usize),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(v) => write!(f, "{}", v),
        }
    }
}

pub fn parse(input: &str) -> Expr {
    let mut lexer = MathLexer::new(input);

    match lexer.next() {
        Some(Token::Num(v)) => Expr::Num(v),
        _ => panic!("Invalid Token"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_num() {
        let expr = parse("42");
        assert_eq!(expr.to_string(), "42");
    }
}
