use std::collections::VecDeque;
use std::fs;
use std::iter::FromIterator;

const PREAMBULE: usize = 25;

struct FixedVec<T> {
    size: usize,
    inner: VecDeque<T>,
}

impl<T> FixedVec<T> {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Self {
            size,
            inner: VecDeque::new(),
        }
    }

    pub fn add(&mut self, item: T) {
        if self.inner.len() >= self.size {
            let _ = self.inner.pop_front();
        }
        self.inner.push_back(item);
    }

    pub fn pair_iter(&self) -> impl Iterator<Item = (&T, &T)> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(move |(index, a)| self.inner.iter().skip(index + 1).map(move |b| (a, b)))
    }
}

fn solve_a(input: &[i64]) -> Option<i64> {
    let mut queue = FixedVec::new(PREAMBULE);
    input[0..PREAMBULE].iter().for_each(|&i| queue.add(i));

    input[PREAMBULE..]
        .iter()
        .find(|&&i| {
            let matched = queue.pair_iter().find(|(&a, &b)| a + b == i);
            if matched.is_none() {
                true
            } else {
                queue.add(i);
                false
            }
        })
        .copied()
}

fn solve_b(input: &[i64], target: i64) -> Option<i64> {
    println!("target: {}", target);
    let (a, b) = (0..input.len()).find_map(|a| {
        input[a..]
            .iter()
            .enumerate()
            .scan(0, |state, (index, el)| {
                *state += el;
                Some((index, *state))
            })
            .take_while(|(_, total)| *total <= target)
            .find(|(_, total)| *total == target)
            .map(|(index, _)| (a, a + index))
    })?;

    let mut clone = Vec::from_iter(input[a..=b].iter().copied());
    clone.sort_unstable();

    Some(clone[0] + clone[clone.len() - 1])
}

fn main() {
    let data = fs::read_to_string("data/day09.txt").unwrap();
    let input = data
        .lines()
        .map(|l| l.parse::<i64>().ok())
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let value_a = solve_a(&input).unwrap();
    // let value_b = solve_b(&input, value_a).unwrap();
    let value_b = solve_b(&input, value_a).unwrap();

    println!("Task A: {}\nTask B: {}", value_a, value_b);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut fixed_vec = FixedVec::new(2);
        fixed_vec.add(5);
        fixed_vec.add(10);
        fixed_vec.add(15);

        assert_eq!(fixed_vec.inner, vec![10, 15]);
    }

    #[test]
    fn test_iter() {
        let mut fixed_vec = FixedVec::new(3);
        fixed_vec.add(5);
        fixed_vec.add(10);
        fixed_vec.add(15);

        let pairs: Vec<_> = fixed_vec.pair_iter().collect();
        assert_eq!(pairs, vec![(&5, &10), (&5, &15), (&10, &15)]);
    }
}
