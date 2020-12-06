use std::collections::HashSet;
use std::fs;

struct Group {
    answers: HashSet<char>,
}

impl Group {
    pub fn parse(input: &str) -> Self {
        let mut answers = HashSet::new();

        for c in input.chars() {
            match c {
                'a'..='z' => {
                    answers.insert(c);
                }
                _ => {}
            }
        }

        Self { answers }
    }

    pub fn any_ansered(&self) -> usize {
        self.answers.len()
    }
}

fn main() {
    let data = fs::read_to_string("data/day06.txt").expect("File not found");

    let foo: usize = data
        .split("\n\n")
        .map(Group::parse)
        .map(|group| group.any_ansered())
        .sum();

    dbg!(foo);
}
