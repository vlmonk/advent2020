use advent2020::grid::Grid;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::Sub;

trait Point: Clone + Eq + core::hash::Hash {
    fn from_xy(x: isize, y: isize) -> Self;
    fn around(&self) -> Box<dyn Iterator<Item = Self>>;
    fn extend_min(&mut self, p: &Self);
    fn extend_max(&mut self, p: &Self);
}

trait DimensionBound {
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn one() -> Self;
    fn all(&self, hrs: &Self) -> Box<dyn Iterator<Item = Self>>;
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

const AROUND: [isize; 3] = [-1, 0, 1];

impl Point3 {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point3 { x, y, z }
    }
}

impl Point4 {
    pub fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Point4 { x, y, z, w }
    }
}

impl DimensionBound for Point3 {
    fn add(&self, hrs: &Self) -> Self {
        Self::new(self.x + hrs.x, self.y + hrs.y, self.z + hrs.z)
    }

    fn sub(&self, hrs: &Self) -> Self {
        Self::new(self.x - hrs.x, self.y - hrs.y, self.z - hrs.z)
    }

    fn one() -> Self {
        Self::new(1, 1, 1)
    }

    fn all(&self, hrs: &Self) -> Box<dyn Iterator<Item = Self>> {
        let min_x = self.x;
        let max_x = hrs.x;

        let min_y = self.y;
        let max_y = hrs.y;

        let min_z = self.z;
        let max_z = hrs.z;

        let iter = (min_x..=max_x)
            .into_iter()
            .map(move |x| (min_y..=max_y).into_iter().map(move |y| (x, y)))
            .flatten()
            .map(move |(x, y)| (min_z..=max_z).into_iter().map(move |z| (x, y, z)))
            .flatten()
            .map(|(x, y, z)| {
                // println!("{}/{}/{}", x, y, z);
                Point3::new(x, y, z)
            });
        Box::new(iter)
    }
}

impl DimensionBound for Point4 {
    fn add(&self, hrs: &Self) -> Self {
        Self::new(
            self.x + hrs.x,
            self.y + hrs.y,
            self.z + hrs.z,
            self.w + hrs.w,
        )
    }

    fn sub(&self, hrs: &Self) -> Self {
        Self::new(
            self.x - hrs.x,
            self.y - hrs.y,
            self.z - hrs.z,
            self.w - hrs.w,
        )
    }

    fn one() -> Self {
        Self::new(1, 1, 1, 1)
    }

    fn all(&self, hrs: &Self) -> Box<dyn Iterator<Item = Self>> {
        let min_x = self.x;
        let max_x = hrs.x;

        let min_y = self.y;
        let max_y = hrs.y;

        let min_z = self.z;
        let max_z = hrs.z;

        let min_w = self.w;
        let max_w = hrs.w;

        let iter = (min_x..=max_x)
            .into_iter()
            .map(move |x| (min_y..=max_y).into_iter().map(move |y| (x, y)))
            .flatten()
            .map(move |(x, y)| (min_z..=max_z).into_iter().map(move |z| (x, y, z)))
            .flatten()
            .map(move |(x, y, z)| (min_w..=max_w).into_iter().map(move |w| (x, y, z, w)))
            .flatten()
            .map(|(x, y, z, w)| Point4::new(x, y, z, w));

        Box::new(iter)
    }
}

impl Point for Point3 {
    fn from_xy(x: isize, y: isize) -> Self {
        Self { x, y, z: 0 }
    }

    fn around(&self) -> Box<dyn Iterator<Item = Self>> {
        let px = self.x;
        let py = self.y;
        let pz = self.z;

        let iter = AROUND
            .iter()
            .map(|x| AROUND.iter().map(move |y| (*x, *y)))
            .flatten()
            .map(|(x, y)| AROUND.iter().map(move |z| (x, y, *z)))
            .flatten()
            .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
            .map(move |(x, y, z)| Point3::new(x + px, y + py, z + pz));

        Box::new(iter)
    }

    fn extend_min(&mut self, p: &Self) {
        self.x = self.x.min(p.x);
        self.y = self.y.min(p.y);
        self.z = self.z.min(p.z);
    }

    fn extend_max(&mut self, p: &Self) {
        self.x = self.x.max(p.x);
        self.y = self.y.max(p.y);
        self.z = self.z.max(p.z);
    }
}

impl Point for Point4 {
    fn from_xy(x: isize, y: isize) -> Self {
        Self { x, y, z: 0, w: 0 }
    }

    fn around(&self) -> Box<dyn Iterator<Item = Self>> {
        let px = self.x;
        let py = self.y;
        let pz = self.z;
        let pw = self.w;

        let iter = AROUND
            .iter()
            .map(|x| AROUND.iter().map(move |y| (*x, *y)))
            .flatten()
            .map(|(x, y)| AROUND.iter().map(move |z| (x, y, *z)))
            .flatten()
            .map(|(x, y, z)| AROUND.iter().map(move |w| (x, y, z, *w)))
            .flatten()
            .filter(|(x, y, z, w)| *x != 0 || *y != 0 || *z != 0 || *w != 0)
            .map(move |(x, y, z, w)| Point4::new(x + px, y + py, z + pz, w + pw));

        Box::new(iter)
    }

    fn extend_min(&mut self, p: &Self) {
        self.x = self.x.min(p.x);
        self.y = self.y.min(p.y);
        self.z = self.z.min(p.z);
        self.w = self.w.min(p.w);
    }

    fn extend_max(&mut self, p: &Self) {
        self.x = self.x.max(p.x);
        self.y = self.y.max(p.y);
        self.z = self.z.max(p.z);
        self.w = self.w.max(p.w);
    }
}

impl Sub<isize> for Point3 {
    type Output = Self;
    fn sub(self, other: isize) -> Self {
        Point3::new(self.x - other, self.y - other, self.z - other)
    }
}

#[derive(Debug)]
struct Dimension<T>
where
    T: Point + DimensionBound,
{
    min: T,
    max: T,
}

impl<T> Dimension<T>
where
    T: Point + DimensionBound + Clone,
{
    pub fn extend(&mut self, point: &T) {
        self.min.extend_min(point);
        self.max.extend_max(point);
    }

    pub fn from_point(point: &T) -> Self {
        Self {
            min: point.clone(),
            max: point.clone(),
        }
    }

    pub fn points(&self) -> Box<dyn Iterator<Item = T>> {
        let min = self.min.sub(&T::one());
        let max = self.max.add(&T::one());

        min.all(&max)
    }
}

// impl Dimension<Point3> {
//     pub fn points(&self) -> Box<dyn Iterator<Item = Point3>> {
//         let min_x = self.min.x - 1;
//         let max_x = self.max.x + 1;

//         let min_y = self.min.y - 1;
//         let max_y = self.max.y + 1;

//         let min_z = self.max.z - 1;
//         let max_z = self.max.z + 1;

//         let iter = (min_x..=max_x)
//             .into_iter()
//             .map(move |x| (min_y..=max_y).into_iter().map(move |y| (x, y)))
//             .flatten()
//             .map(move |(x, y)| (min_z..=max_z).into_iter().map(move |z| (x, y, z)))
//             .flatten()
//             .map(|(x, y, z)| Point3::new(x, y, z));
//         Box::new(iter)
//     }
// }

#[derive(Debug)]
struct Space<T>
where
    T: Point + DimensionBound,
{
    world: HashSet<T>,
    dimension: Option<Dimension<T>>,
}

impl<T> Space<T>
where
    T: Point + DimensionBound + Clone + 'static,
{
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
                    let point = T::from_xy(x as isize, y as isize);
                    space.add_point(point);
                    // dbg!(&space.dimension);
                }
            }
        }

        space
    }

    fn add_point(&mut self, point: T) {
        match self.dimension.as_mut() {
            Some(d) => d.extend(&point),
            None => self.dimension = Some(Dimension::from_point(&point)),
        }

        self.world.insert(point);
    }

    fn active(&self, p: &T) -> bool {
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

    fn points(&self) -> Box<dyn Iterator<Item = T>> {
        if let Some(d) = &self.dimension {
            d.points()
        } else {
            Box::new(std::iter::empty::<T>())
        }
    }
}

impl fmt::Display for Space<Point3> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(d) = self.dimension.as_ref() {
            writeln!(
                f,
                "Dimension: X: {}:{}, Y: {}:{}, Z: {}:{}",
                d.min.x, d.max.x, d.min.y, d.max.y, d.min.z, d.max.z
            );

            for z in d.min.z..=d.max.z {
                write!(f, "Layer {}\n", z);
                for y in d.min.y..=d.max.y {
                    for x in d.min.x..=d.max.x {
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
    let mut space: Space<Point3> = Space::from_grid(grid);

    for _ in 0..6 {
        // println!("{}", space);
        space = space.step();
    }

    dbg!(space.total());

    let grid = Grid::parse(&raw, parser).ok_or("can't parse input")?;
    let mut space: Space<Point4> = Space::from_grid(grid);

    for _ in 0..6 {
        // println!("{}", space);
        space = space.step();
    }

    dbg!(space.total());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter_total() {
        let point = Point3::new(0, 0, 0);
        let total = point.around().count();

        assert_eq!(total, 26);
    }

    #[test]
    fn test_dimension_points() {
        let min = Point3::new(0, 0, 0);
        let max = Point3::new(0, 0, 0);
        let d = Dimension { min, max };

        assert_eq!(d.points().count(), 27);
    }

    #[test]
    fn test_dimension_points_more() {
        let min = Point3::new(0, 0, 0);
        let max = Point3::new(1, 2, 3);
        let d = Dimension { min, max };

        assert_eq!(d.points().count(), 120);
    }
}
