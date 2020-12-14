use std::convert::TryFrom;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn step(&mut self, dir: &Direction, amount: isize) {
        match dir {
            Direction::North => self.y -= amount,
            Direction::East => self.x += amount,
            Direction::South => self.y += amount,
            Direction::West => self.x -= amount,
        };
    }

    fn step_by(&mut self, next: &Self) {
        self.x += next.x;
        self.y += next.y;
    }

    fn times(&self, amount: isize) -> Self {
        Self {
            x: self.x * amount,
            y: self.y * amount,
        }
    }

    fn turn(&mut self) {
        let x = -self.y;
        let y = self.x;

        self.x = x;
        self.y = y;
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug)]
enum Action {
    Move(Direction, isize),
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
            'N' => Ok(Action::Move(Direction::North, value)),
            'S' => Ok(Action::Move(Direction::South, value)),
            'E' => Ok(Action::Move(Direction::East, value)),
            'W' => Ok(Action::Move(Direction::West, value)),
            'L' => Ok(Action::Left(value)),
            'R' => Ok(Action::Right(value)),
            'F' => Ok(Action::Forward(value)),
            _ => Err(format!("Invalid input: {}", input).into()),
        }
    }
}

#[derive(Debug)]
struct World {
    ship: Point,
    waypoint: Point,
    dir: Direction,
}

impl World {
    fn new() -> Self {
        Self {
            ship: Point::new(0, 0),
            waypoint: Point::new(10, -1),
            dir: Direction::East,
        }
    }

    fn distance_from_start(&self) -> isize {
        self.ship.x.abs() + self.ship.y.abs()
    }

    fn step(&mut self, action: &Action) {
        match action {
            Action::Move(dir, amount) => self.ship.step(dir, *amount),
            Action::Forward(value) => self.ship.step(&self.dir, *value),
            Action::Left(value) => self.turn(*value * -1),
            Action::Right(value) => self.turn(*value),
        }
    }

    fn step_relative(&mut self, action: &Action) {
        match action {
            Action::Move(dir, amount) => self.waypoint.step(dir, *amount),
            Action::Forward(value) => self.ship.step_by(&self.waypoint.times(*value)),
            Action::Left(value) => self.turn_waypoint(*value * -1),
            Action::Right(value) => self.turn_waypoint(*value),
        }
    }

    fn turn(&mut self, angle: isize) {
        let angle = ((angle / 90) % 4 + 4) % 4;

        for _ in 0..angle {
            self.dir = self.dir.turn()
        }
    }

    fn turn_waypoint(&mut self, angle: isize) {
        let angle = ((angle / 90) % 4 + 4) % 4;

        for _ in 0..angle {
            self.waypoint.turn()
        }
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

    println!("Task A: {}", world.distance_from_start());

    let mut world = World::new();
    for action in input.iter() {
        world.step_relative(action);
    }

    println!("Task B: {}", world.distance_from_start());

    Ok(())
}
