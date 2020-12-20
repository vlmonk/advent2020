mod direction;
mod grid;

use direction::Direction;
use grid::Grid;
use std::error::Error;
use std::{fmt, fs};

#[derive(PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Seat::Floor => write!(f, "."),
            Seat::Empty => write!(f, "L"),
            Seat::Occupied => write!(f, "#"),
        }
    }
}

fn parser(input: char) -> Option<Seat> {
    match input {
        'L' => Some(Seat::Empty),
        '.' => Some(Seat::Floor),
        _ => None,
    }
}

fn step_task_a(grid: &Grid<Seat>, x: usize, y: usize) -> Seat {
    let occupied = Direction::all()
        .filter_map(|d| d.iter(x, y).next())
        .filter_map(|(x, y)| grid.get(x, y))
        .filter(|s| **s == Seat::Occupied)
        .count();

    match grid.get(x as usize, y as usize).unwrap() {
        Seat::Empty => {
            if occupied == 0 {
                Seat::Occupied
            } else {
                Seat::Empty
            }
        }
        Seat::Occupied => {
            if occupied >= 4 {
                Seat::Empty
            } else {
                Seat::Occupied
            }
        }
        Seat::Floor => Seat::Floor,
    }
}

fn step_task_b(grid: &Grid<Seat>, x: usize, y: usize) -> Seat {
    let occupied = Direction::all()
        .filter_map(|d| {
            d.iter(x, y)
                .scan((), |(), (x, y)| grid.get(x, y))
                .fuse()
                .skip_while(|s| **s == Seat::Floor)
                .next()
        })
        .filter(|s| **s == Seat::Occupied)
        .count();

    match grid.get(x as usize, y as usize).unwrap() {
        Seat::Empty => {
            if occupied == 0 {
                Seat::Occupied
            } else {
                Seat::Empty
            }
        }
        Seat::Occupied => {
            if occupied >= 5 {
                Seat::Empty
            } else {
                Seat::Occupied
            }
        }
        Seat::Floor => Seat::Floor,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data/day11.txt")?;
    let mut grid = Grid::parse(&data, parser).ok_or("parsing error")?;

    loop {
        let changed = grid.step(step_task_a);

        if changed == 0 {
            break;
        }
    }

    let task_a = grid.iter().filter(|item| **item == Seat::Occupied).count();
    dbg!(task_a);

    let mut grid = Grid::parse(&data, parser).ok_or("parsing error")?;

    loop {
        let changed = grid.step(step_task_b);

        if changed == 0 {
            break;
        }
    }

    let task_b = grid.iter().filter(|item| **item == Seat::Occupied).count();
    dbg!(task_b);

    Ok(())
}
