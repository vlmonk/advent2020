use advent2020::grid::Grid;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

const AROUND: [isize; 3] = [-1, 0, 1];

impl Point3 {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point3 { x, y, z }
    }

    pub fn around(&self) -> impl Iterator<Item = Point3> {
        let px = self.x;
        let py = self.y;
        let pz = self.z;
        AROUND
            .iter()
            .map(|x| AROUND.iter().map(move |y| (*x, *y)))
            .flatten()
            .map(|(x, y)| AROUND.iter().map(move |z| (x, y, *z)))
            .flatten()
            .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
            .map(move |(x, y, z)| Point3::new(x + px, y + py, z + pz))
    }
}

#[derive(Debug)]
struct Dimension {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl Dimension {
    pub fn extend(&mut self, point: &Point3) {
        self.min_x = self.min_x.min(point.x - 1);
        self.max_x = self.max_x.max(point.x + 1);
        self.min_y = self.min_y.min(point.y - 1);
        self.max_y = self.max_y.max(point.y + 1);
        self.min_z = self.min_z.min(point.z - 1);
        self.max_z = self.max_z.max(point.z + 1);
    }

    pub fn from_point(point: &Point3) -> Self {
        Self {
            min_x: point.x - 1,
            max_x: point.x + 1,
            min_y: point.y - 1,
            max_y: point.y + 1,
            min_z: point.z - 1,
            max_z: point.z + 1,
        }
    }
}

#[derive(Debug)]
struct Space {
    world: HashSet<Point3>,
    dimension: Option<Dimension>,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(d) = self.dimension.as_ref() {
            writeln!(
                f,
                "Dimension: X: {}:{}, Y: {}:{}, Z: {}:{}",
                d.min_x, d.max_x, d.min_y, d.max_y, d.min_z, d.max_z
            );

            for z in d.min_z + 1..d.max_z {
                write!(f, "Layer {}\n", z);
                for y in d.min_y + 1..d.max_y {
                    for x in d.min_x + 1..d.max_x {
                        if self.active(&Point3::new(x, y, z)) {
                            write!(f, "#");
                        } else {
                            write!(f, ".");
                        }
                    }

                    write!(f, "\n");
                }
            }

            Ok(())
        } else {
            write!(f, "Empty fields")
        }
    }
}

impl Space {
    fn new() -> Self {
        Self {
            world: HashSet::new(),
            dimension: None,
        }
    }

    pub fn from_grid(input: Grid<Element>) -> Self {
        let mut space = Space::new();
        for x in 0..input.width {
            for y in 0..input.height {
                if let Some(Element::Active) = input.get(x, y) {
                    let point = Point3::new(x as isize, y as isize, 0);
                    space.add_point(point);
                    // dbg!(&space.dimension);
                }
            }
        }
        space
    }

    fn add_point(&mut self, point: Point3) {
        match self.dimension.as_mut() {
            Some(d) => d.extend(&point),
            None => self.dimension = Some(Dimension::from_point(&point)),
        }

        self.world.insert(point);
    }

    fn active(&self, p: &Point3) -> bool {
        self.world.contains(p)
    }

    fn total(&self) -> usize {
        self.world.len()
    }

    pub fn step(&self) -> Self {
        let mut space = Space::new();

        for point in self.points() {
            let around = point.around().filter(|p| self.active(p)).count();

            if self.active(&point) {
                if around == 2 || around == 3 {
                    space.add_point(point)
                }
            } else {
                if around == 3 {
                    space.add_point(point)
                }
            }
        }

        // dbg!(&self);
        space
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point3>> {
        if let Some(d) = &self.dimension {
            let min_x = d.min_x;
            let max_x = d.max_x;

            let min_y = d.min_y;
            let max_y = d.max_y;

            let min_z = d.min_z;
            let max_z = d.max_z;

            let iter = (min_x..=max_x)
                .into_iter()
                .map(move |x| (min_y..=max_y).into_iter().map(move |y| (x, y)))
                .flatten()
                .map(move |(x, y)| (min_z..=max_z).into_iter().map(move |z| (x, y, z)))
                .flatten()
                .map(|(x, y, z)| Point3::new(x, y, z));
            Box::new(iter)
        } else {
            Box::new(std::iter::empty::<Point3>())
        }
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
    let mut space = Space::from_grid(grid);

    println!("before:\n{}", space);

    for step in 0..6 {
        space = space.step();
        // dbg!(&space);
    }

    dbg!(space.total());

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
