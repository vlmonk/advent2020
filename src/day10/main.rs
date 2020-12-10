use advent2020::measure;
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

fn solve_b(input: &[isize]) -> Result<isize, Box<dyn Error>> {
    let max = *input.last().ok_or_else(|| "Last not found")?;
    let last = std::iter::once(max + 3);
    let first = std::iter::once(0);

    let input = first
        .chain(input.iter().map(|v| *v))
        .chain(last)
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let v = permutate_rec(input.as_slice(), &mut cache, 0);

    Ok(v)
}

fn permutate_rec(
    input: &[isize],
    cache: &mut HashMap<usize, isize>,
    index_from_start: usize,
) -> isize {
    if let Some(i) = cache.get(&index_from_start) {
        return *i;
    }

    if input.len() == 1 {
        cache.insert(index_from_start, 1);
        return 1;
    }

    let start = input[0];

    let result = input
        .iter()
        .enumerate()
        .skip(1)
        .take_while(|(_, v)| **v - start <= 3)
        .map(|(index, v)| {
            // println!("{} -> {} (index {})", start, v, index);

            permutate_rec(&input[index..], cache, index_from_start + index)
        })
        .sum();

    cache.insert(index_from_start, result);

    return result;
}

fn main() -> Result<(), Box<dyn Error>> {
    let (result, elapsed) = measure(|| -> Result<(isize, isize), Box<dyn Error>> {
        let input = read_input()?;
        let task_a = solve_a(&input)?;
        let task_b = solve_b(&input)?;

        Ok((task_a, task_b))
    });

    match result {
        Ok((a, b)) => {
            println!("task A: {}\ntask B: {}\nTotal time: {}μs ", a, b, elapsed);
        }
        _ => {}
    }

    Ok(())
}
