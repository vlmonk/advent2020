use advent2020::measure;
use std::collections::{HashMap, HashSet};
use std::fs;

type ContainRules<'a> = HashMap<&'a str, HashSet<&'a str>>;
type NumberRules<'a> = HashMap<&'a str, HashSet<(&'a str, usize)>>;

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

            let num_end = trimmed.find(' ')?;
            let bag_start = trimmed.find("bag")?;

            let count: usize = trimmed[0..num_end].parse().ok()?;
            let bag = &trimmed[num_end + 1..bag_start - 1];

            Some((bag, count))
        })
        .collect::<Option<Vec<_>>>()?;

    Some((bag, inner))
}

fn parse(input: &str) -> Option<(ContainRules, NumberRules)> {
    let mut contain = HashMap::new();
    let mut number = HashMap::new();

    for line in input.lines() {
        let (bag, inners) = parse_line(line)?;

        let mut contain_rule = HashSet::new();
        let mut number_rule = HashSet::new();

        for inner in inners.into_iter() {
            contain_rule.insert(inner.0);
            number_rule.insert(inner);
        }

        contain.insert(bag, contain_rule);
        number.insert(bag, number_rule);
    }

    Some((contain, number))
}

fn solve_a(rules: &ContainRules, target: &str) -> usize {
    let mut cache: HashMap<&str, bool> = HashMap::new();

    rules
        .keys()
        .filter(|k| is_contain(k, target, rules, &mut cache))
        .count()
}

fn is_contain<'a, 'b>(
    cargo: &'a str,
    target: &'a str,
    rules: &'a ContainRules,
    cache: &'b mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(v) = cache.get(cargo) {
        return *v;
    }

    let rule = rules.get(cargo).unwrap();

    if rule.len() == 0 {
        cache.insert(cargo, false);
        return false;
    }

    if rule.contains(target) {
        cache.insert(cargo, true);
        return true;
    }

    let result = rule
        .iter()
        .filter(|n| is_contain(n, target, rules, cache))
        .count()
        > 0;

    cache.insert(cargo, result);

    result
}

fn solve_b(rules: &NumberRules, target: &str) -> usize {
    let mut cache: HashMap<&str, usize> = HashMap::new();
    calculate_b(rules, &target, &mut cache) - 1
}

fn calculate_b<'a, 'b>(
    rules: &'a NumberRules,
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
        let (contain, numbers) = parse(&data).expect("cant parse input");

        let a = solve_a(&contain, "shiny gold");
        let b = solve_b(&numbers, "shiny gold");

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
