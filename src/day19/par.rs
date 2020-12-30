use crate::lex::{Lex, LexerIter};

type ParseResult<T> = Option<(T, usize)>;

#[derive(Debug, PartialEq)]
struct Rule {
    id: usize,
    body: RuleBody,
}

#[derive(Debug, PartialEq)]
enum RuleBody {
    Term(char),
    Refs(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
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
    let part_a = parse_refs(input)?;

    None
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
        assert_eq!(
            rule,
            Rule {
                id: 0,
                body: RuleBody::Term('b')
            }
        );
    }

    #[test]
    fn parse_ref() {
        let input = "4: 1 2";
        let rule = parse_rule(&input);
        assert_eq!(
            rule,
            Rule {
                id: 4,
                body: RuleBody::Refs(vec![1, 2])
            }
        );
    }

    #[test]
    fn parse_or() {
        let input = "3: 4 5 | 5 4";
        let rule = parse_rule(&input);
        assert_eq!(
            rule,
            Rule {
                id: 4,
                body: RuleBody::Or(vec![4, 5], vec![5, 4])
            }
        );
    }
}
