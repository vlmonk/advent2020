#[derive(Debug)]
pub struct Grid<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: std::fmt::Debug + std::cmp::PartialEq,
{
    pub fn step<F>(&mut self, changer: F) -> usize
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
                    dbg!(current, &next_item);
                    changed += 1
                }
                next.push(next_item);
            }
        }

        self.data = next;
        changed
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);

        let index = x + y * self.width;
        &self.data[index]
    }

    pub fn parse<F>(input: &str, parser: F) -> Option<Grid<T>>
    where
        F: Fn(char) -> Option<T>,
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
}
