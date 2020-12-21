use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Ranges {
    first: Range,
    second: Range,
}

type Ticket = Vec<usize>;

#[derive(Debug)]
struct GameInput {
    ranges: HashMap<String, Ranges>,
    fields: usize,
    my: Ticket,
    nearby: Vec<Ticket>,
}

fn parge_range(input: &str) -> Option<(String, Ranges)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    let caps = RE.captures(input)?;
    let name = caps.get(1).map(|c| String::from(c.as_str()))?;

    let from = caps.get(2).and_then(|c| c.as_str().parse::<usize>().ok())?;
    let to = caps.get(3).and_then(|c| c.as_str().parse::<usize>().ok())?;
    let first = Range { from, to };

    let from = caps.get(4).and_then(|c| c.as_str().parse::<usize>().ok())?;
    let to = caps.get(5).and_then(|c| c.as_str().parse::<usize>().ok())?;
    let second = Range { from, to };

    let ranges = Ranges { first, second };
    Some((name, ranges))
}

fn parse_ticket(input: &str) -> Option<Ticket> {
    input.split(',').map(|v| v.parse().ok()).collect()
}

impl TryFrom<&str> for GameInput {
    type Error = Box<dyn Error>;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut sections = input.split("\n\n");
        let ranges = sections
            .next()
            .and_then(|section| {
                section
                    .lines()
                    .map(parge_range)
                    .collect::<Option<HashMap<_, _>>>()
            })
            .ok_or("invald input")?;

        dbg!(&ranges);

        let my = sections
            .next()
            .and_then(|section| section.lines().nth(1))
            .and_then(parse_ticket)
            .ok_or("Invalid input: my ticket")?;

        dbg!(&my);

        let nearby = sections
            .next()
            .and_then(|section| {
                section
                    .lines()
                    .skip(1)
                    .map(parse_ticket)
                    .collect::<Option<Vec<_>>>()
            })
            .ok_or("Invalid input: nearby ticket")?;

        dbg!(&nearby);

        let fields = ranges.len();

        Ok(GameInput {
            ranges,
            my,
            nearby,
            fields,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string("data/day16.txt")?;
    let input = GameInput::try_from(raw.as_ref())?;
    dbg!(input);

    Ok(())
}
