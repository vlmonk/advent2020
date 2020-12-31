mod lex;
mod par;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string("data/day19.txt")?;
    let mut section = raw.split("\n\n");

    let rules_raw = section.nth(0).ok_or("Invalid input")?;
    let rules = rules_raw.lines().map(|l| par::parse(l)).collect::<Vec<_>>();

    for rule in rules.iter() {
        println!("{}", rule);
    }

    Ok(())
}
