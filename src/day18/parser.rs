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

fn op_power(op: &Op) -> (u8, u8) {
    match op {
        Op::Add => (1, 2),
    }
}

pub fn parse_expr(lexer: &mut MathLexer, min_pb: u8) -> Expr {
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

        let (l_pb, r_pb) = op_power(&op);

        if l_pb < min_pb {
            break;
        }

        lexer.next();
        let rhs = parse_expr(lexer, r_pb);
        lhs = Expr::Add(Box::new(lhs), Box::new(rhs));
    }

    lhs
}

pub fn parse(input: &str) -> Expr {
    let mut lexer = MathLexer::new(input);
    parse_expr(&mut lexer, 0)
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

    #[test]
    fn test_parse_associativity() {
        let expr = parse("5 + 6 + 1");
        assert_eq!(expr.to_string(), "(+ (+ 5 6) 1)")
    }
}
