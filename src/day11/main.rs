mod grid;

use grid::Grid;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn parser(input: char) -> Option<Seat> {
    match input {
        'L' => Some(Seat::Empty),
        '.' => Some(Seat::Floor),
        _ => None,
    }
}

fn step(grid: &Grid<Seat>, x: usize, y: usize) -> Seat {
    let x = x as isize;
    let y = y as isize;

    let around = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];

    let filtered: Vec<_> = around
        .into_iter()
        .filter(|(x, y)| {
            *x >= 0 && *x < grid.width as isize && *y >= 0 && *y < grid.height as isize
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect();

    match grid.get(x as usize, y as usize) {
        Seat::Empty => {
            let occupied_count = filtered
                .iter()
                .filter(|(x, y)| grid.get(*x, *y) == &Seat::Occupied)
                .count();

            if occupied_count == 0 {
                Seat::Occupied
            } else {
                Seat::Empty
            }
        }
        Seat::Occupied => {
            let occupied_count = filtered
                .iter()
                .filter(|(x, y)| grid.get(*x, *y) == &Seat::Occupied)
                .count();

            if occupied_count >= 4 {
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
    let mut steps = 0;

    dbg!(grid.height);
    dbg!(grid.width);

    loop {
        let changed = grid.step(step);
        dbg!(steps, changed);
        if changed == 0 {
            break;
        }

        steps += 1;
    }

    Ok(())
}
