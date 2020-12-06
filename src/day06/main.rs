use std::collections::HashSet;
use std::fs;

struct Group {
    group_answers: HashSet<char>,
}

impl Group {
    pub fn parse(input: &str) -> Self {
        let mut group_answers = HashSet::new();

        for c in input.chars() {
            match c {
                'a'..='z' => {
                    group_answers.insert(c);
                }
                _ => {}
            }
        }

        Self { group_answers }
    }

    pub fn any_ansered(&self) -> usize {
        self.group_answers.len()
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
