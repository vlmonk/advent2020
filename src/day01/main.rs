use std::fs;

struct Repeater {
    data: Vec<i32>,
}

const TARGET: i32 = 2020;

impl Repeater {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    fn pair(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.pair_from(0)
    }

    fn pair_from(&self, n: usize) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.from_enum(n)
            .flat_map(move |(i, &a)| self.from(i + 1).map(move |&b| (a, b)))
    }

    fn triple(&self) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
        self.from_enum(0)
            .flat_map(move |(i, &a)| self.pair_from(i + 1).map(move |(b, c)| (a, b, c)))
    }

    fn from(&self, n: usize) -> impl Iterator<Item = &i32> + '_ {
        self.data[n..].iter()
    }

    fn from_enum(&self, n: usize) -> impl Iterator<Item = (usize, &i32)> + '_ {
        self.data[n..].iter().enumerate()
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .filter_map(|i| i.parse::<i32>().ok())
        .collect()
}

fn main() {
    let input = fs::read_to_string("data/day01.txt").unwrap();
    let data = parse_input(&input);
    let repeater = Repeater::new(data);

    let task_a = repeater
        .pair()
        .find(|(a, b)| a + b == TARGET)
        .map(|(a, b)| a * b);

    let task_b = repeater
        .triple()
        .find(|(a, b, c)| a + b + c == TARGET)
        .map(|(a, b, c)| a * b * c);

    dbg!(task_a);
    dbg!(task_b);
}
