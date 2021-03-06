use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Bus {
    Active(usize),
    Canceled,
}

impl TryFrom<&str> for Bus {
    type Error = Box<dyn Error>;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input == "x" {
            return Ok(Bus::Canceled);
        }

        if let Ok(v) = input.parse::<usize>() {
            Ok(Bus::Active(v))
        } else {
            Err("Invalid input".into())
        }
    }
}

#[derive(Debug)]
pub struct Input {
    start_time: usize,
    buses: Vec<Bus>,
}

impl TryFrom<&str> for Input {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut lines = input.lines();
        let start_time = lines
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or("Invalid input")?;

        let buses = lines
            .next()
            .ok_or_else(|| "Invalid input".into())
            .and_then(|l| {
                l.split(',')
                    .map(Bus::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })?;

        Ok(Self { start_time, buses })
    }
}

mod solver {
    pub fn solve_a(input: &crate::Input) -> Option<usize> {
        (input.start_time..)
            .find_map(|time| {
                input.buses.iter().find_map(|item| match item {
                    crate::Bus::Active(n) if time % n == 0 => Some(((time - input.start_time), n)),
                    _ => None,
                })
            })
            .map(|(a, b)| {
                dbg!(a);
                dbg!(b);
                a * b
            })
    }

    pub fn solve_b(input: &crate::Input) -> Option<usize> {
        let target = input
            .buses
            .iter()
            .enumerate()
            .filter_map(|(index, v)| {
                if let crate::Bus::Active(n) = v {
                    Some((index, *n))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let (mut start, mut step) = target.get(0)?;

        for i in target[1..].iter() {
            let result = solve_b_pair(start, step, *i);
            start = result.0;
            step = result.1;
        }

        Some(start)
    }

    fn solve_b_pair(start: usize, step: usize, next: (usize, usize)) -> (usize, usize) {
        let (a, b) = next;
        let mut i = start;

        loop {
            if (i + a) % b == 0 {
                return (i, step * b);
            }

            i += step;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string("data/day13.txt")?;
    let input: Input = raw[..].try_into()?;

    let task_a = solver::solve_a(&input).ok_or("Task A not solved")?;
    dbg!(task_a);

    let task_b = solver::solve_b(&input).ok_or("Task B not solved")?;
    dbg!(task_b);

    Ok(())
}
