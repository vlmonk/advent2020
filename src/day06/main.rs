use std::collections::HashSet;
use std::fs;

struct Group {
    group_answers: HashSet<char>,
    people: Vec<HashSet<char>>,
}

impl Group {
    pub fn parse(input: &str) -> Self {
        let mut group_answers = HashSet::new();
        let mut people = vec![];

        for p in input.lines() {
            let mut person = HashSet::new();

            for c in p.chars() {
                match c {
                    'a'..='z' => {
                        group_answers.insert(c);
                        person.insert(c);
                    }
                    _ => {}
                }
            }

            if person.len() > 0 {
                people.push(person);
            }
        }

        Self {
            group_answers,
            people,
        }
    }

    pub fn any_answered(&self) -> usize {
        self.group_answers.len()
    }

    pub fn all_answered(&self) -> usize {
        let total = self.any_answered();

        self.group_answers
            .iter()
            .filter(|answer| self.answered_to(answer) == total)
            .count()
    }

    fn answered_to(&self, answer: &char) -> usize {
        self.people
            .iter()
            .filter(|person| person.contains(answer))
            .count()
    }
}

fn main() {
    let data = fs::read_to_string("data/day06.txt").expect("File not found");

    let groups = data.split("\n\n").map(Group::parse).collect::<Vec<_>>();

    let task_a: usize = groups.iter().map(|g| g.any_answered()).sum();
    let task_b: usize = groups.iter().map(|g| g.all_answered()).sum();

    dbg!(task_a);
    dbg!(task_b);
}
