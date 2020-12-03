use std::convert::TryFrom;
use std::error;
use std::fs;

#[derive(Debug)]
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
        let mut lines = input.lines();
        let first_line = lines.next()?;
        let chars: Result<Vec<_>, _> = first_line.chars().map(Object::try_from).collect();
        dbg!(chars);

        return None;
    }
}

impl Field for GameField {
    fn get(&self, x: usize, y: usize) -> Option<Object> {
        return None;
    }
}

fn main() {
    let input = fs::read_to_string("data/day03.txt").unwrap();
    let field = GameField::parse(&input).unwrap();

    let point = field.get(0, 0);
    dbg!(point);
}
