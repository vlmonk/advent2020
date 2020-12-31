mod lex;
mod par;
mod ruleset;

use ruleset::RuleSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string("data/day19.txt")?;
    let mut section = raw.split("\n\n");

    let rules_raw = section.next().ok_or("Invalid input")?;
    let mut set = RuleSet::parse(rules_raw);

    let lines = section
        .next()
        .ok_or("Invalid input")?
        .lines()
        .collect::<Vec<_>>();

    let total = lines.iter().filter(|l| set.check(l)).count();
    println!("Task A: {}", total);

    set.patch("8: 42 | 42 8");
    set.patch("11: 42 31 | 42 11 31");

    let total = lines.iter().filter(|l| set.check(l)).count();
    println!("Task B: {}", total);

    Ok(())
}
