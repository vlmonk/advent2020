use std::slice::Iter;

#[derive(PartialEq)]
pub enum Direction {
    NN,
    NE,
    EE,
    SE,
    SS,
    SW,
    WW,
    NW,
}

impl Direction {
    pub fn all() -> Iter<'static, Direction> {
        static DIR: [Direction; 8] = [
            Direction::NN,
            Direction::NE,
            Direction::EE,
            Direction::SE,
            Direction::SS,
            Direction::SW,
            Direction::WW,
            Direction::NW,
        ];
        DIR.iter()
    }

    pub fn iter(&self, x: usize, y: usize) -> PairIter {
        PairIter {
            direction: &self,
            x,
            y,
        }
    }

    fn is_west(&self) -> bool {
        *self == Direction::NW || *self == Direction::WW || *self == Direction::SW
    }

    fn is_east(&self) -> bool {
        *self == Direction::NE || *self == Direction::EE || *self == Direction::SE
    }

    fn is_north(&self) -> bool {
        *self == Direction::NW || *self == Direction::NN || *self == Direction::NE
    }

    fn is_south(&self) -> bool {
        *self == Direction::SW || *self == Direction::SS || *self == Direction::SE
    }
}

pub struct PairIter<'a> {
    direction: &'a Direction,
    x: usize,
    y: usize,
}

impl<'a> Iterator for PairIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 && self.direction.is_west() {
            return None;
        }

        if self.y == 0 && self.direction.is_north() {
            return None;
        }

        if self.direction.is_north() {
            self.y -= 1;
        } else if self.direction.is_south() {
            self.y += 1;
        }

        if self.direction.is_west() {
            self.x -= 1;
        } else if self.direction.is_east() {
            self.x += 1;
        }

        Some((self.x, self.y))
    }
}
