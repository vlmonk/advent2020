use crate::lex::{Lex, LexerIter};
use std::fmt;
use std::iter::Peekable;

fn format_refs(input: &[usize]) -> String {
    input
        .iter()
        .map(|v| format!("{}", v))
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub id: usize,
    pub body: RuleBody,
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.body)
    }
}

#[derive(Debug, PartialEq)]
pub enum RuleBody {
    Term(char),
    Refs(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

impl fmt::Display for RuleBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleBody::Term(c) => write!(f, "'{}'", c),
            RuleBody::Refs(v) => write!(f, "{}", format_refs(v)),
            RuleBody::Or(a, b) => write!(f, "{} | {}", format_refs(a), format_refs(b)),
        }
    }
}

fn parse_id(input: &mut Peekable<LexerIter>) -> usize {
    match input.next() {
        Some(Lex::Num(id)) => id,
        _ => panic!("Invalid token"),
    }
}

fn parse_ref_list(input: &mut Peekable<LexerIter>) -> Vec<usize> {
    let first = match input.next() {
        Some(Lex::Num(n)) => n,
        _ => panic!("Invalid token"),
    };

    let mut result = vec![first];

    loop {
        match input.peek() {
            Some(Lex::Num(n)) => {
                result.push(*n);
                input.next();
            }
            _ => break,
        }
    }

    result
}

fn parse_refs(input: &mut Peekable<LexerIter>) -> RuleBody {
    let a = parse_ref_list(input);

    match input.next() {
        None => RuleBody::Refs(a),
        Some(Lex::Pipe) => {
            let b = parse_ref_list(input);
            assert_eq!(input.next(), None);
            RuleBody::Or(a, b)
        }
        _ => panic!("Invalid token"),
    }
}

// fn parse_or(input: &[Lex]) -> ParseResult<RuleBody> {
//     let (part_a, n) = parse_refs(input)?;
//     let _ = parse_pipe(&input[n..])?;
//     let (part_b, m) = parse_refs(&input[n + 1..])?;

//     None
//     // Some((RuleBody::Or(part_a, part_b), m))
// }

fn parse_body(input: &mut Peekable<LexerIter>) -> RuleBody {
    match input.peek() {
        Some(Lex::Char(c)) => {
            let body = RuleBody::Term(*c);
            input.next();
            body
        }
        Some(Lex::Num(_)) => parse_refs(input),
        _ => panic!("Invalid token"),
    }
}

pub fn parse(input: &str) -> Rule {
    let mut lex = LexerIter::new(input).peekable();
    let id = parse_id(&mut lex);
    assert_eq!(lex.next(), Some(Lex::Column));
    let body = parse_body(&mut lex);
    Rule { id, body }
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_rule(input: &str) -> Rule {
        parse(&input)
    }

    #[test]
    fn parse_simple() {
        let input = r#"0: "b""#;
        let rule = parse_rule(&input);

        assert_eq!("0: 'b'", rule.to_string());
    }

    #[test]
    fn parse_ref() {
        let input = "4: 1    2";
        let rule = parse_rule(&input);

        assert_eq!("4: 1 2", rule.to_string());
    }

    #[test]
    fn parse_or() {
        let input = "3: 4 5 | 5 4";
        let rule = parse_rule(&input);

        assert_eq!("3: 4 5 | 5 4", rule.to_string());
    }
}
