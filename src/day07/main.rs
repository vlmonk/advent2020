use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bag(String);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Inner(Bag, usize);

type RuleSet = HashMap<Bag, HashSet<Inner>>;

impl Bag {
    fn new(input: &str) -> Self {
        Self(input.into())
    }
}

impl Inner {
    fn new(bag: Bag, count: usize) -> Self {
        Self(bag, count)
    }
}

fn parse_line(input: &str) -> Option<(Bag, Vec<Inner>)> {
    let mut parts = input.split(" bags contain ");
    let bag_part = parts.next()?;
    let bag = Bag::new(bag_part);

    let inner_part = parts.next()?;

    if inner_part.contains("no other") {
        return Some((bag, vec![]));
    }

    let inner = inner_part
        .split(",")
        .map(|part| {
            let trimmed = part.trim();
            // dbg!(trimmed);

            let num_end = trimmed.find(' ')?;
            let bag_start = trimmed.find("bag")?;
            // dbg!(num_end);
            // dbg!(bag_start);

            let count: usize = trimmed[0..num_end].parse().ok()?;
            // dbg!(count);

            let bag = Bag::new(&trimmed[num_end + 1..bag_start - 1]);
            // dbg!(&bag);

            Some(Inner::new(bag, count))
        })
        .collect::<Option<Vec<_>>>()?;

    Some((bag, inner))
}

fn parse(input: &str) -> Option<RuleSet> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        // dbg!(&line);
        let (bag, inners) = parse_line(line)?;
        // dbg!(&bag);
        // dbg!(&inners);

        let mut rule = HashSet::new();
        for inner in inners.into_iter() {
            rule.insert(inner);
        }

        rules.insert(bag, rule);
    }

    Some(rules)
}

fn solve_a(rules: &RuleSet, target: &str) -> usize {
    let mut founed = HashSet::new();
    founed.insert(Bag::new(target));

    loop {
        let mut next = HashSet::new();
        for (rule, inner) in rules.iter() {
            for el in inner {
                for target in founed.iter() {
                    if &el.0 == target {
                        // dbg!(format!("{:?} -> {:?}", &rule, &el));
                        // dbg!(&founed);
                        if !founed.contains(&rule) {
                            next.insert(rule.clone());
                        }
                    }
                }
            }
        }

        // dbg!(&next);

        if next.len() == 0 {
            // dbg!(&next);
            break;
        }

        for i in next.drain() {
            founed.insert(i);
        }
    }
    return founed.len() - 1;
}

fn main() {
    let data = fs::read_to_string("data/day07.txt").unwrap();
    let rules = parse(&data).expect("cant parse input");

    let a = solve_a(&rules, "shiny gold");
    dbg!(a);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let bug = Bag::new("light red");
        let inner = vec![
            Inner::new(Bag::new("bright white"), 1),
            Inner::new(Bag::new("muted yellow"), 2),
        ];
        assert_eq!(parse_line(input).unwrap(), (bug, inner));
    }

    #[test]
    fn test_parse_empty() {
        let input = "faded blue bags contain no other bags.";
        let bug = Bag::new("faded blue");
        let inner = vec![];
        assert_eq!(parse_line(input).unwrap(), (bug, inner));
        r
    }
}
