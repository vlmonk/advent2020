use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn read_input() -> Result<Vec<isize>, Box<dyn Error>> {
    let raw = fs::read_to_string("data/day10.txt")?;
    let mut result = raw
        .lines()
        .map(|line| line.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;
    result.sort();
    Ok(result)
}

fn solve_a(input: &[isize]) -> Result<isize, Box<dyn Error>> {
    let max = *input.last().ok_or_else(|| "Last not found")?;
    let last = std::iter::once(max + 3);
    let seq = input.iter().map(|&v| v).chain(last);

    let mut summary = HashMap::new();

    seq.scan(0, |state, b| {
        let next = b - *state;
        *state = b;
        Some(next)
    })
    .for_each(|v| {
        let entry = summary.entry(v).or_insert(0);
        *entry += 1;
    });

    let result =
        summary.get(&1).map(|v| *v).unwrap_or(0) * summary.get(&3).map(|v| *v).unwrap_or(0);

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;
    let task_a = solve_a(&input)?;

    dbg!(task_a);

    Ok(())
}
