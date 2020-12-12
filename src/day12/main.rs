use std::convert::TryFrom;
use std::error::Error;

enum Direction {
    North,
    East,
    South,
    West,
}

enum Action {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl TryFrom<&str> for Action {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let action = input.chars().next().ok_or("Empty input")?;
        let value = input[1..].parse::<isize>()?;

        match action {
            'N' => Ok(Action::North(value)),
            'S' => Ok(Action::South(value)),
            'E' => Ok(Action::East(value)),
            'W' => Ok(Action::West(value)),
            'L' => Ok(Action::Left(value)),
            'R' => Ok(Action::Right(value)),
            'F' => Ok(Action::Forward(value)),
            _ => Err(format!("Invalid input: {}", input)),
        }
    }
}

fn main() {
    println!("Day 12 placeholder")
}
