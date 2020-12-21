use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Seen {
    Once(usize),
    Twice { last: usize, before: usize },
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

        let seen = match self.memory.get(&value) {
            Some(Seen::Once(last)) => Seen::Twice {
                before: *last,
                last: self.turn,
            },
            Some(Seen::Twice { last, .. }) => Seen::Twice {
                before: *last,
                last: self.turn,
            },
            _ => Seen::Once(self.turn),
        };

        self.memory.insert(value, seen);
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

fn main() {
    println!("Day 15 TODO");
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
}
