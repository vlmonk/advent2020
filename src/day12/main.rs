use std::convert::TryFrom;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
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
            _ => Err(format!("Invalid input: {}", input).into()),
        }
    }
}

#[derive(Debug)]
struct World {
    x: isize,
    y: isize,
    dir: Direction,
}

impl World {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::East,
        }
    }

    fn step(&mut self, action: &Action) {
        // let foo: [i8; 4] = [0, 1, 2, 3];
        // let bar = foo[8];

        match action {
            Action::North(value) => self.y -= value,
            Action::South(value) => self.y += value,
            Action::East(value) => self.x += value,
            Action::West(value) => self.x -= value,
            Action::Forward(value) => self.step_forward(*value),
            Action::Left(value) => self.turn(*value * -1),
            Action::Right(value) => self.turn(*value),
        }
    }

    fn step_forward(&mut self, value: isize) {
        let next_action = match self.dir {
            Direction::North => Action::North(value),
            Direction::East => Action::East(value),
            Direction::South => Action::South(value),
            Direction::West => Action::West(value),
        };

        self.step(&next_action);
    }

    fn turn(&mut self, angle: isize) {
        let before = match self.dir {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };

        let angle = (angle / 90) % 4;
        let next_angel = ((((before + angle) % 4) + 4) % 4) as usize;
        self.dir = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ][next_angel];
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data/day12.txt")?;
    let input = data
        .lines()
        .map(Action::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let mut world = World::new();

    for action in input.iter() {
        world.step(action);
    }

    Ok(())
}
