use std::collections::{HashMap, VecDeque};

struct Game {
    init: VecDeque<usize>,
    memory: HashMap<usize, usize>,
    turn: usize,
    last: Option<usize>
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.turn += 1;
        let init = match self.init.pop_front() {
            Some(init) => init,
            None => 0
        }


        Some(init)
    }
}

impl Game {
    pub fn new(input: &[usize]) -> Self {
        Self {
            init: input.iter().copied().collect(),
            memory: HashMap::new(),
            turn: 0,
            last: None
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
