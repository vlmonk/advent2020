use std::convert::TryFrom;
use std::error;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Object {
    Empty,
    Tree,
}

impl TryFrom<char> for Object {
    type Error = Box<dyn error::Error>;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Tree),
            _ => Err("Invalid character".into()),
        }
    }
}

trait Field {
    fn get(&self, x: usize, y: usize) -> Option<Object>;
}

struct GameField {
    width: usize,
    height: usize,
    objects: Vec<Vec<Object>>,
}

impl GameField {
    pub fn parse(input: &str) -> Option<Self> {
        let objects: Vec<Vec<_>> = input
            .lines()
            .map(|l| l.chars().map(Object::try_from).collect())
            .collect::<Result<Vec<Vec<_>>, _>>()
            .ok()?;

        let height = objects.len();
        let width = objects.get(0).map(|line| line.len())?;

        return Some(Self {
            objects,
            height,
            width,
        });
    }
}

impl Field for GameField {
    fn get(&self, x: usize, y: usize) -> Option<Object> {
        if y >= self.height {
            return None;
        }

        let normalized_x = x % self.width;
        Some(self.objects[y][normalized_x])
    }
}

struct StepIterator {
    next_x: usize,
    next_y: usize,
}

impl StepIterator {
    pub fn new() -> Self {
        Self {
            next_x: 0,
            next_y: 0,
        }
    }
}

impl Iterator for StepIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.next_x;
        let y = self.next_y;

        self.next_x += 3;
        self.next_y += 1;

        Some((x, y))
    }
}

fn main() {
    let input = fs::read_to_string("data/day03.txt").unwrap();
    let field = GameField::parse(&input).unwrap();
    let steps = StepIterator::new();

    let foo: usize = steps
        .scan((), |(), (x, y)| field.get(x, y))
        .map(|o| if let Object::Tree = o { 1 } else { 0 })
        .sum();

    dbg!(foo);
}
