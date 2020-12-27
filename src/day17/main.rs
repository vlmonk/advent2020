use advent2020::grid::Grid;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

struct Space {
    world: HashSet<Point>,
    min_x: Option<isize>,
    max_x: Option<isize>,
    min_y: Option<isize>,
    max_y: Option<isize>,
    min_z: Option<isize>,
    max_z: Option<isize>,
}

impl Space {
    pub fn new() -> Self {
        Self {
            world: HashSet::new(),
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            min_z: None,
            max_z: None,
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.min_x = if let Some(min_x) = self.min_x {
            Some(min_x.min(point.x))
        } else {
            Some(point.x)
        };

        self.world.insert(point);
    }
}

#[derive(PartialEq)]
enum Element {
    Active,
    Inactive,
}

fn parser(input: char) -> Option<Element> {
    match input {
        '#' => Some(Element::Active),
        '.' => Some(Element::Inactive),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string("data/day17.txt")?;
    let grid = Grid::parse(&raw, parser).ok_or("can't parse input")?;

    Ok(())
}
