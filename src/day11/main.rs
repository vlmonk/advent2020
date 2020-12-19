use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Grid<T>
where
    T: std::fmt::Debug,
{
    data: Vec<T>,
    width: usize,
    height: usize,
}

// impl Grid {}

fn parse_grid<T, F>(input: &str, parser: F) -> Option<Grid<T>>
where
    F: Fn(char) -> Option<T>,
    T: std::fmt::Debug,
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

#[derive(Debug)]
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

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data/day11.txt")?;
    let grid = parse_grid(&data, parser);

    Ok(())
}
