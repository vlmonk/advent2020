use advent2020::grid::Grid;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn extend_min(a: Option<isize>, b: isize) -> Option<isize> {
    let value = if let Some(a) = a { a } else { b };
    Some(value.min(b - 1))
}

fn extend_max(a: Option<isize>, b: isize) -> Option<isize> {
    let value = if let Some(a) = a { a } else { b };
    Some(value.max(b + 1))
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

const AROUND: [isize; 3] = [-1, 0, 1];

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn around(&self) -> impl Iterator<Item = Point> {
        AROUND
            .iter()
            .map(|x| AROUND.iter().map(move |y| (*x, *y)))
            .flatten()
            .map(|(x, y)| AROUND.iter().map(move |z| (x, y, *z)))
            .flatten()
            .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
            .map(|(x, y, z)| Point::new(x, y, z))
    }
}

#[derive(Debug)]
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

    pub fn from_grid(input: Grid<Element>) -> Self {
        let mut space = Space::new();
        for x in 0..input.width {
            for y in 0..input.height {
                if let Some(Element::Active) = input.get(x, y) {
                    let point = Point::new(x as isize, y as isize, 0);
                    space.add_point(point);
                }
            }
        }
        space
    }

    pub fn add_point(&mut self, point: Point) {
        self.min_x = extend_min(self.min_x, point.x);
        self.min_y = extend_min(self.min_y, point.y);
        self.min_z = extend_min(self.min_z, point.z);

        self.max_x = extend_max(self.max_x, point.x);
        self.max_y = extend_max(self.max_y, point.y);
        self.max_z = extend_max(self.min_z, point.z);

        self.world.insert(point);
    }
}

#[derive(PartialEq, Debug)]
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
    let space = Space::from_grid(grid);

    dbg!(space);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter_total() {
        let point = Point::new(0, 0, 0);
        let total = point.around().count();

        assert_eq!(total, 26);
    }
}
