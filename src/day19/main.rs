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
    let set = RuleSet::parse(rules_raw);

    let lines = section
        .next()
        .ok_or("Invalid input")?
        .lines()
        .collect::<Vec<_>>();

    let total = lines.iter().filter(|l| set.check(l)).count();

    println!("Task A: {}", total);

    Ok(())
}
