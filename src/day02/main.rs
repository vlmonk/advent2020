use advent2020::measure;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
struct Policy {
    min: usize,
    max: usize,
    char: char,
    list: Vec<char>,
}

impl Policy {
    pub fn new(min: usize, max: usize, char: char, list: Vec<char>) -> Self {
        Self {
            min,
            max,
            char,
            list,
        }
    }

    pub fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        }

        let cap = RE.captures(input)?;
        let min = cap.get(1).and_then(|s| s.as_str().parse::<usize>().ok())?;
        let max = cap.get(2).and_then(|s| s.as_str().parse::<usize>().ok())?;
        let char = cap.get(3).and_then(|s| s.as_str().chars().next())?;
        let list = cap.get(4).map(|s| s.as_str().chars().collect())?;

        Some(Self::new(min, max, char, list))
    }

    pub fn valid_a(&self) -> bool {
        let count = self.list.iter().filter(|&&c| c == self.char).count();
        count >= self.min && count <= self.max
    }

    pub fn valid_b(&self) -> bool {
        let len = self.list.len();

        if self.min > len || self.max > len {
            return false;
        }

        let a = self.list[self.min - 1] == self.char;
        let b = self.list[self.max - 1] == self.char;

        a ^ b
    }
}

fn main() {
    let ((task_a, task_b), elapsed) = measure(|| {
        let input = fs::read_to_string("data/day02.txt").unwrap();

        let data: Vec<_> = input
            .lines()
            .filter_map(|line| Policy::parse(line))
            .collect();

        let task_a = data.iter().filter(|&policy| policy.valid_a()).count();
        let task_b = data.iter().filter(|&policy| policy.valid_b()).count();
        (task_a, task_b)
    });

    println!(
        "task A: {}\ntask B: {}\nTotal time: {}μs ",
        task_a, task_b, elapsed
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "1-3 a: abcde";
        let expected = Policy::new(1, 3, 'a', vec!['a', 'b', 'c', 'd', 'e']);
        assert_eq!(Policy::parse(input), Some(expected));
    }

    #[test]
    fn test_valid_a() {
        let input = Policy::parse("1-3 a: abcde").unwrap();
        assert_eq!(input.valid_a(), true);
    }

    #[test]
    fn test_not_valid_a() {
        let input = Policy::parse("1-3 b: cdefg").unwrap();
        assert_eq!(input.valid_a(), false);
    }

    #[test]
    fn test_valid_b() {
        let input = Policy::parse("1-3 a: abcde").unwrap();
        assert_eq!(input.valid_b(), true);
    }

    #[test]
    fn test_not_valid_b() {
        let input = Policy::parse("2-9 c: ccccccccc").unwrap();
        assert_eq!(input.valid_b(), false);
    }
}
