use crate::lex::{Lex, LexerIter};
use std::fmt;

type ParseResult<T> = Option<(T, usize)>;

fn format_refs(input: &[usize]) -> String {
    input
        .iter()
        .map(|v| format!("{}", v))
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Debug, PartialEq)]
struct Rule {
    id: usize,
    body: RuleBody,
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.body)
    }
}

#[derive(Debug, PartialEq)]
enum RuleBody {
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

fn parse_id(input: &[Lex]) -> ParseResult<usize> {
    match input[0] {
        Lex::Num(id) => Some((id, 1)),
        _ => None,
    }
}

fn parse_column(input: &[Lex]) -> ParseResult<()> {
    match input[0] {
        Lex::Column => Some(((), 1)),
        _ => None,
    }
}

fn parse_pipe(input: &[Lex]) -> Option<()> {
    match input[0] {
        Lex::Pipe => Some(()),
        _ => None,
    }
}

fn parse_term(input: &[Lex]) -> ParseResult<RuleBody> {
    match input[0] {
        Lex::Char(c) => Some((RuleBody::Term(c), 1)),
        _ => None,
    }
}

fn parse_refs(input: &[Lex]) -> ParseResult<RuleBody> {
    let mut refs = vec![];
    let mut total = 0;

    if let Lex::Num(v) = input[0] {
        refs.push(v);
        total += 1
    } else {
        return None;
    }

    for el in input[1..].iter() {
        if let Lex::Num(v) = el {
            total += 1;
            refs.push(*v);
        } else {
            break;
        }
    }

    Some((RuleBody::Refs(refs), total))
}

fn parse_or(input: &[Lex]) -> ParseResult<RuleBody> {
    let (part_a, n) = parse_refs(input)?;
    let _ = parse_pipe(&input[n..])?;
    let (part_b, m) = parse_refs(&input[n + 1..])?;

    None
    // Some((RuleBody::Or(part_a, part_b), m))
}

fn parse_body(input: &[Lex]) -> ParseResult<RuleBody> {
    parse_term(input)
        .or_else(|| parse_refs(input))
        .or_else(|| parse_or(input))
}

fn parse(input: &[Lex]) -> Option<Rule> {
    let (id, _) = parse_id(&input)?;
    let _ = parse_column(&input[1..])?;
    let (body, _) = parse_body(&input[2..])?;
    Some(Rule { id, body })
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_rule(input: &str) -> Rule {
        let tokens = LexerIter::new(&input).collect::<Vec<_>>();
        parse(&tokens).unwrap()
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
