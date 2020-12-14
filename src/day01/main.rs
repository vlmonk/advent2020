use advent2020::measure;
use std::fs;

struct Repeater {
    data: Vec<i32>,
}

const TARGET: i32 = 2020;

impl Repeater {
    pub fn new(mut data: Vec<i32>) -> Self {
        data.sort_unstable();
        Self { data }
    }

    fn pair(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.pair_from(0)
    }

    fn pair_from(&self, n: usize) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.from_enum(n).flat_map(move |(i, &a)| {
            self.from(i + 1)
                .take_while(move |&b| a + b <= TARGET)
                .map(move |&b| (a, b))
        })
    }

    fn triple(&self) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
        self.from_enum(0).flat_map(move |(i, &a)| {
            self.pair_from(i + 1)
                .take_while(move |(b, c)| a + b + c <= TARGET)
                .map(move |(b, c)| (a, b, c))
        })
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
        .split('\n')
        .filter_map(|i| i.parse::<i32>().ok())
        .collect()
}

fn main() {
    let ((task_a, task_b), elapsed) = measure(|| {
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

        (task_a, task_b)
    });

    println!(
        "task A: {}\ntask B: {}\nTotal time: {}μs ",
        task_a.unwrap(),
        task_b.unwrap(),
        elapsed
    );
}
