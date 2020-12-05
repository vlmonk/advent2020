use advent2020::measure;
use std::fs;

fn binary(input: &str, one: char, zero: char) -> Option<u32> {
    let size = input.len();
    let mut result: u32 = 0;

    for (i, c) in input.chars().enumerate() {
        let shift = size - 1 - i;
        match c {
            c if c == one => result |= 1 << shift,
            c if c == zero => {}
            _ => return None,
        }
    }

    Some(result)
}

#[derive(PartialEq, Debug)]
struct BoardPass {
    row: u32,
    column: u32,
}

impl BoardPass {
    fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }

    pub fn parse(input: &str) -> Option<Self> {
        let row = binary(&input[0..7], 'B', 'F')?;
        let column = binary(&input[7..10], 'R', 'L')?;

        Some(Self::new(row, column))
    }

    fn seatid(&self) -> u32 {
        self.row * 8 + self.column
    }
}

struct PairIter<I, K> {
    inner: K,
    last: Option<I>,
}

impl<I, K> PairIter<I, K>
where
    K: Iterator<Item = I>,
{
    pub fn new(inner: K) -> Self {
        Self { inner, last: None }
    }
}

impl<I, K> Iterator for PairIter<I, K>
where
    K: Iterator<Item = I>,
    I: Copy,
{
    type Item = (I, I);
    fn next(&mut self) -> Option<Self::Item> {
        match self.last.take() {
            Some(a) => {
                let b = self.inner.next()?;
                self.last = Some(b);
                Some((a, b))
            }
            _ => {
                let a = self.inner.next()?;
                let b = self.inner.next()?;

                self.last = Some(b);
                Some((a, b))
            }
        }
    }
}

fn main() {
    let ((task_a, task_b), elapsed) =
        measure(|| {
            let data = fs::read_to_string("data/day05.txt").unwrap();
            let board_passes = data
                .lines()
                .map(BoardPass::parse)
                .collect::<Option<Vec<_>>>()
                .expect("bad input");

            let mut numbers: Vec<_> = board_passes.iter().map(|pass| pass.seatid()).collect();
            numbers.sort();
            let task_a = numbers.iter().max().map(|v| *v);
            let task_b = PairIter::new(numbers.iter()).find_map(|(a, b)| {
                if *b == *a + 1 {
                    None
                } else {
                    Some(a + 1)
                }
            });

            (task_a, task_b)
        });

    println!(
        "task A: {}\ntask B: {}\nTotal time: {}μs ",
        task_a.unwrap_or(0),
        task_b.unwrap_or(0),
        elapsed
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary() {
        assert_eq!(binary("10", '1', '0'), Some(2));
        assert_eq!(binary("1*0", '1', '0'), None);
        assert_eq!(binary("FBFBBFF", 'B', 'F'), Some(44));
    }

    #[test]
    fn test_board_pass() {
        assert_eq!(
            BoardPass::parse("BFFFBBFRRR").unwrap(),
            BoardPass::new(70, 7)
        );
    }
}
