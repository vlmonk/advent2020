use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Grid<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    fn step<F>(&mut self, changer: F) -> usize
    where
        F: Fn(&Grid<T>, usize, usize) -> T,
    {
        let mut next = vec![];
        let mut changed = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let current = self.get(x, y);
                let next_item = changer(&self, x, y);
                if *current != next_item {
                    changed += 1
                }
                next.push(next_item);
            }
        }

        self.data = next;
        changed
    }

    fn get(&self, x: usize, y: usize) -> &T {
        assert!(x <= self.width);
        assert!(y <= self.height);

        let index = x + y * self.width;
        &self.data[index]
    }
}

fn parse_grid<T, F>(input: &str, parser: F) -> Option<Grid<T>>
where
    F: Fn(char) -> Option<T>,
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    let mut width: Option<usize> = None;
    let mut height = 0;
    let mut data = vec![];

    for line in input.lines() {
        let mut points: Vec<_> = line.chars().map(&parser).collect::<Option<Vec<_>>>()?;
        if let Some(width) = width {
            if points.len() != width {
                return None;
            }
        } else {
            width = Some(points.len())
        }

        data.append(&mut points);
        height += 1;
    }

    width.map(|width| Grid {
        width,
        height,
        data,
    })
}

#[derive(Debug, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn parser(input: char) -> Option<Seat> {
    dbg!(input);
    match input {
        'L' => Some(Seat::Empty),
        '.' => Some(Seat::Floor),
        _ => None,
    }
}

fn step(grid: &Grid<Seat>, x: usize, y: usize) -> Seat {
    Seat::Empty
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data/day11.txt")?;
    let mut grid = parse_grid(&data, parser).ok_or("parsing error")?;
    let mut steps = 0;

    loop {
        let changed = grid.step(step);
        if changed == 0 {
            break;
        }

        steps += 1;
    }

    dbg!(steps);

    Ok(())
}
