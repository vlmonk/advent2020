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

impl Range {
    fn include(&self, input: usize) -> bool {
        input >= self.from && input <= self.to
    }
}

#[derive(Debug)]
struct Ranges {
    first: Range,
    second: Range,
}

impl Ranges {
    fn include(&self, value: usize) -> bool {
        self.first.include(value) || self.second.include(value)
    }
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

        let my = sections
            .next()
            .and_then(|section| section.lines().nth(1))
            .and_then(parse_ticket)
            .ok_or("Invalid input: my ticket")?;

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

        let fields = ranges.len();

        Ok(GameInput {
            ranges,
            my,
            nearby,
            fields,
        })
    }
}

struct Solver<'a> {
    game: &'a GameInput,
}

impl<'a> Solver<'a> {
    fn new(game: &'a GameInput) -> Self {
        Self { game }
    }

    fn task_a(&self) -> usize {
        self.game
            .nearby
            .iter()
            .flat_map(|v| v.iter())
            .filter(|v| !self.is_valid_field(**v))
            .sum()
    }

    fn task_b(&self) -> usize {
        let valid = self
            .game
            .nearby
            .iter()
            .filter(|ticket| self.is_valid_ticket(ticket))
            .collect::<Vec<_>>();

        println!("total: {}, valid: {}", self.game.nearby.len(), valid.len());

        let foo = valid.iter().map(|t| t[0]).collect::<Vec<_>>();
        dbg!(&foo);

        let bar = self
            .game
            .ranges
            .iter()
            .filter(|(_, range)| foo.iter().all(|value| range.include(*value)))
            .collect::<Vec<_>>();

        dbg!(bar);

        5
    }

    fn is_valid_field(&self, value: usize) -> bool {
        self.game.ranges.iter().any(|(_, v)| v.include(value))
    }

    fn is_valid_ticket(&self, ticket: &Ticket) -> bool {
        ticket.iter().all(|v| self.is_valid_field(*v))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string("data/day16.txt")?;
    let input = GameInput::try_from(raw.as_ref())?;
    let solver = Solver::new(&input);

    let task_a = solver.task_a();
    let task_b = solver.task_b();

    println!("Task A: {}\nTask B: {}", task_a, task_b);
    Ok(())
}
