use advent2020::measure;
use std::collections::{HashMap, HashSet};
use std::fs;

type RuleSet<'a> = HashMap<&'a str, HashSet<(&'a str, usize)>>;

fn parse_line(input: &str) -> Option<(&str, Vec<(&str, usize)>)> {
    let mut parts = input.split(" bags contain ");
    let bag = parts.next()?;

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

            let bag = &trimmed[num_end + 1..bag_start - 1];
            // dbg!(&bag);

            Some((bag, count))
        })
        .collect::<Option<Vec<_>>>()?;

    Some((bag, inner))
}

fn parse(input: &str) -> Option<RuleSet> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        // dbg!(&line);
        let (bag, inners) = parse_line(line)?;
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
    founed.insert(target);

    loop {
        let mut next = HashSet::new();
        for (rule, inner) in rules.iter() {
            for el in inner {
                for target in founed.iter() {
                    if &el.0 == target {
                        // dbg!(format!("{:?} -> {:?}", &rule, &el));
                        // dbg!(&founed);
                        if !founed.contains(rule) {
                            next.insert(rule);
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

fn solve_b(rules: &RuleSet, target: &str) -> usize {
    let mut cache: HashMap<&str, usize> = HashMap::new();
    calculate_b(rules, &target, &mut cache) - 1
}

fn calculate_b<'a, 'b>(
    rules: &'a RuleSet,
    target: &'a str,
    cache: &'b mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(v) = cache.get(target) {
        // dbg!(format!("founc cached {} -> {}", target.0, v));
        return *v;
    }

    let rule = rules.get(target).unwrap();
    // dbg!(&rule);

    let value: usize = rule
        .iter()
        .map(|inner| calculate_b(rules, &inner.0, cache) * inner.1)
        .sum::<usize>()
        + 1;

    // dbg!(format!("calculated {} -> {}", target.0, value));

    cache.insert(target, value);

    value
}

fn main() {
    let ((a, b), elapsed) = measure(|| {
        let data = fs::read_to_string("data/day07.txt").unwrap();
        let rules = parse(&data).expect("cant parse input");

        //     let mut x: HashMap<&str, usize> = HashMap::new();
        //     let sss = "foofoo";
        //     let a = &sss[0..3];
        //     let b = &sss[3..6];

        //     dbg!(a);
        //     dbg!(b);

        //     x.insert(a, 100);
        //     dbg!(x.get(b));

        let a = solve_a(&rules, "shiny gold");
        let b = solve_b(&rules, "shiny gold");

        (a, b)
    });

    println!("task A: {}\ntask B: {}\nTotal time: {}μs ", a, b, elapsed);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let bug = "light red";
        let inner = vec![("bright white", 1), ("muted yellow", 2)];
        assert_eq!(parse_line(input).unwrap(), (bug, inner));
    }

    #[test]
    fn test_parse_empty() {
        let input = "faded blue bags contain no other bags.";
        let bug = "faded blue";
        let inner = vec![];
        assert_eq!(parse_line(input).unwrap(), (bug, inner));
    }
}
