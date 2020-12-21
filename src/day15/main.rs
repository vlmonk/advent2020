use std::collections::{HashMap, VecDeque};
use std::default::Default;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Seen {
    Never,
    Once(usize),
    Twice { last: usize, before: usize },
}

impl Default for Seen {
    fn default() -> Self {
        Self::Never
    }
}

impl Seen {
    pub fn see(&mut self, turn: usize) {
        match self {
            Seen::Never => *self = Seen::Once(turn),
            Seen::Once(last) => {
                *self = Seen::Twice {
                    before: *last,
                    last: turn,
                }
            }
            Seen::Twice { last, .. } => {
                *self = Seen::Twice {
                    before: *last,
                    last: turn,
                }
            }
        }
    }
}

struct Game {
    init: VecDeque<usize>,
    memory: HashMap<usize, Seen>,
    turn: usize,
    last: Option<usize>,
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.turn += 1;

        let value = match self.init.pop_front() {
            Some(init) => init,
            None => {
                let last = self.last.unwrap_or(0);
                let seen = self.memory.get(&last);

                if let Some(Seen::Twice { last, before }) = seen {
                    last - before
                } else {
                    0
                }
            }
        };

        self.last = Some(value);

        self.memory
            .entry(value)
            .or_insert_with(Seen::default)
            .see(self.turn);

        Some(value)
    }
}

impl Game {
    pub fn new(input: &[usize]) -> Self {
        Self {
            init: input.iter().copied().collect(),
            memory: HashMap::new(),
            turn: 0,
            last: None,
        }
    }
}

fn parse_input(input: &str) -> Option<Vec<usize>> {
    let first_line = input.lines().next()?;
    first_line
        .split(',')
        .map(|v| v.parse::<usize>().ok())
        .collect::<Option<Vec<_>>>()
}

const TARGET_A: usize = 2020;
const TARGET_B: usize = 30000000;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data/day15.txt")?;
    let input = parse_input(&data).ok_or("Invalid input")?;
    let mut game = Game::new(&input);

    let task_a = game.nth(TARGET_A - 1).ok_or("Task A not found")?;
    let task_b = game
        .nth(TARGET_B - TARGET_A - 1)
        .ok_or("Task A not found")?;

    println!("Task A: {}\nTask B: {}", task_a, task_b);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intial() {
        let input = vec![0, 3, 6];
        let iter = Game::new(&input);
        let result = iter.take(10).collect::<Vec<_>>();

        assert_eq!(vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0], result);
    }

    #[test]
    fn test_nth() {
        let input = vec![3, 1, 2];
        let mut iter = Game::new(&input);
        let result = iter.nth(2019);

        assert_eq!(Some(1836), result);
    }
}
