use crate::lexer::{MathLexer, Token};
use std::fmt;

pub enum Expr {
    Num(usize),
    Add(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(v) => write!(f, "{}", v),
            Expr::Add(a, b) => write!(f, "(+ {} {})", a, b),
        }
    }
}

enum Op {
    Add,
}

pub fn parse_expr(lexer: &mut MathLexer) -> Expr {
    let mut lhs = match lexer.next() {
        Some(Token::Num(v)) => Expr::Num(v),
        Some(t) => panic!("bad token: {:?}", t),
        _ => panic!("no input"),
    };

    loop {
        let op = match lexer.peek() {
            Some(Token::Eol) => break,
            Some(Token::Add) => Op::Add,
            Some(t) => panic!("bad token: {:?}", t),
            _ => panic!("no input"),
        };

        lexer.next();
        let rhs = parse_expr(lexer);
        lhs = Expr::Add(Box::new(lhs), Box::new(rhs));
    }

    lhs
}

pub fn parse(input: &str) -> Expr {
    let mut lexer = MathLexer::new(input);
    parse_expr(&mut lexer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_num() {
        let expr = parse("42");
        assert_eq!(expr.to_string(), "42");
    }

    #[test]
    fn test_parse_add() {
        let expr = parse("5+ 6");
        assert_eq!(expr.to_string(), "(+ 5 6)")
    }
}
