use std::collections::VecDeque;
use std::fs;

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

fn main() {
    let data = fs::read_to_string("data/day09.txt").unwrap();
    let input = data
        .lines()
        .map(|l| l.parse::<i64>().ok())
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let mut queue = FixedVec::new(PREAMBULE);
    input[0..PREAMBULE].iter().for_each(|&i| queue.add(i));
    let founed = input[PREAMBULE..].iter().find(|&&i| {
        println!("Test number {}", i);
        let matched = queue.pair_iter().find(|(&a, &b)| a + b == i);
        if let None = matched {
            return true;
        } else {
            queue.add(i);
            return false;
        }
    });
    dbg!(founed);

    println!("Main");
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
