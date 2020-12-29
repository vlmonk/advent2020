use crate::lex::{Lex, LexerIter};

#[derive(Debug, PartialEq)]
struct Rule {
    id: usize,
    body: RuleBody,
}

#[derive(Debug, PartialEq)]
enum RuleBody {
    Term(char),
    Refs(Vec<usize>),
}

fn parse_id(input: &[Lex]) -> Option<usize> {
    match input[0] {
        Lex::Num(id) => Some(id),
        _ => None,
    }
}

fn parse_column(input: &[Lex]) -> Option<()> {
    match input[0] {
        Lex::Column => Some(()),
        _ => None,
    }
}

fn parse_term(input: &[Lex]) -> Option<RuleBody> {
    match input[0] {
        Lex::Char(c) => Some(RuleBody::Term(c)),
        _ => None,
    }
}

fn parse_refs(input: &[Lex]) -> Option<RuleBody> {
    let mut refs = vec![];

    if let Lex::Num(v) = input[0] {
        refs.push(v);
    } else {
        return None;
    }

    for el in input[1..].iter() {
        if let Lex::Num(v) = el {
            refs.push(*v);
        } else {
            break;
        }
    }

    Some(RuleBody::Refs(refs))
}

fn parse_body(input: &[Lex]) -> Option<RuleBody> {
    parse_term(input).or_else(|| parse_refs(input))
}

fn parse(input: &[Lex]) -> Option<Rule> {
    let id = parse_id(&input)?;
    let _ = parse_column(&input[1..])?;
    let body = parse_body(&input[2..])?;
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
}
