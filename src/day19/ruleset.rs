use crate::par::{self, Rule, RuleBody};
use std::collections::HashMap;

pub struct RuleSet {
    inner: HashMap<usize, RuleBody>,
}

impl RuleSet {
    pub fn new(input: Vec<Rule>) -> Self {
        let mut map: HashMap<usize, RuleBody> = HashMap::new();
        for rule in input {
            let Rule { id, body } = rule;
            map.insert(id, body);
        }
        Self { inner: map }
    }
    pub fn parse(input: &str) -> Self {
        let rules = input.lines().map(|l| par::parse(l)).collect::<Vec<_>>();
        RuleSet::new(rules)
    }

    pub fn patch(&mut self, line: &str) {
        let Rule { id, body } = par::parse(line);
        self.inner.insert(id, body);
    }

    pub fn check(&self, input: &str) -> bool {
        let start = self.inner.get(&0).expect("Rule 0 not found");
        let checked = self.check_rule(input, start);

        checked
            .into_iter()
            .find(|res| *res == input.len())
            .is_some()
    }

    pub fn check_rule(&self, input: &str, rule: &RuleBody) -> Vec<usize> {
        match rule {
            RuleBody::Term(c) => match input.chars().next() {
                Some(m) if m == *c => {
                    vec![1]
                }
                _ => {
                    vec![]
                }
            },
            RuleBody::Refs(refs) => self.check_refs(&input, refs),
            RuleBody::Or(a, b) => {
                let mut res_a = self.check_refs(&input, a);
                let mut res_b = self.check_refs(&input, b);

                res_a.append(&mut res_b);
                res_a
            }
        }
    }

    pub fn check_refs(&self, input: &str, refs: &[usize]) -> Vec<usize> {
        let mut current = vec![0];

        for id in refs {
            let mut next = vec![];
            let rule = self.inner.get(id).expect("Rule not found");

            for start in current {
                let step_result = self.check_rule(&input[start..], rule);
                for s in step_result {
                    next.push(s + start);
                }
            }

            current = next;
        }

        current
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_simple() {
        let input = r#"0: "a""#;
        let rules = RuleSet::parse(input);

        assert_eq!(rules.check("a"), true);
        assert_eq!(rules.check("b"), false);
        assert_eq!(rules.check("aa"), false);
    }

    #[test]
    fn test_check_refs() {
        let input = "0: 2 1\n1: \"a\"\n2: \"b\"";
        let rules = RuleSet::parse(input);

        assert_eq!(rules.check("ba"), true);
        assert_eq!(rules.check("bb"), false);
        assert_eq!(rules.check("baa"), false);
    }

    //     #[test]
    //     fn test_check_or() {
    //         let input = r#"0: 1 2
    // 1: "a"
    // 2: 1 3 | 3 1
    // 3: "b" "#;
    //         let rules = RuleSet::parse(input);

    //         assert_eq!(rules.check("aab"), true);
    //         assert_eq!(rules.check("aba"), true);
    //         assert_eq!(rules.check("bab"), false);
    //         assert_eq!(rules.check("abab"), false);
    //     }
}
