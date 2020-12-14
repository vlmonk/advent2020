use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum Command {
    SetMask { and: u64, or: u64, float: u64 },
    SetMem { addr: usize, value: u64 },
}

fn parse_mask(input: &str) -> Result<(u64, u64, u64), Box<dyn Error>> {
    if input.len() != 36 {
        return Err("invalid input len".into());
    }

    let mut and: u64 = 0;
    let mut or: u64 = 0;
    let mut float: u64 = 0;

    for (index, c) in input.chars().enumerate() {
        let offset = 35 - index;

        match c {
            'X' => {
                and |= 1 << offset;
                float |= 1 << offset
            }
            '0' => {}
            '1' => {
                and |= 1 << offset;
                or |= 1 << offset;
            }
            _ => return Err("invalid input".into()),
        }
    }

    Ok((and, or, float))
}

fn parse_mem(input: &str) -> Result<(usize, u64), Box<dyn Error>> {
    let i_start = input.find('[').ok_or("invalid input")?;
    let i_end = input.find(']').ok_or("invalid input")?;
    let eq = input.find('=').ok_or("invalid input")?;

    let addr = input[i_start + 1..i_end].parse::<usize>()?;
    let value = input[eq + 2..].parse::<u64>()?;

    Ok((addr, value))
}

impl TryFrom<&str> for Command {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if &input[0..7] == "mask = " {
            let (and, or, float) = parse_mask(&input[7..])?;
            Ok(Command::SetMask { and, or, float })
        } else if &input[0..3] == "mem" {
            let (addr, value) = parse_mem(&input)?;
            Ok(Command::SetMem { addr, value })
        } else {
            Err("Invalid command".into())
        }
    }
}

fn parse_code(input: &str) -> Result<Vec<Command>, Box<dyn Error>> {
    input.lines().map(Command::try_from).collect()
}

struct Solver {
    and_mask: u64,
    or_mask: u64,
    float: u64,
    memory: HashMap<usize, u64>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            and_mask: 0,
            or_mask: 0,
            float: 0,
            memory: HashMap::new(),
        }
    }

    pub fn run(&mut self, code: &[Command]) {
        for cmd in code.iter() {
            self.run_command(cmd);
        }
    }

    pub fn run_v2(&mut self, code: &[Command]) {
        for cmd in code.iter() {
            self.run_command_v2(cmd);
        }
    }

    pub fn sum(&self) -> u64 {
        self.memory.values().sum()
    }

    fn run_command(&mut self, command: &Command) {
        match command {
            Command::SetMask { and, or, .. } => {
                self.or_mask = *or;
                self.and_mask = *and;
            }
            Command::SetMem { addr, value } => {
                let value = value & self.and_mask | self.or_mask;
                self.memory.insert(*addr, value);
            }
        }
    }

    fn run_command_v2(&mut self, command: &Command) {
        match command {
            Command::SetMask { or, float, .. } => {
                self.or_mask = *or;
                self.float = *float;
            }
            Command::SetMem { addr, value } => {
                let addr = (*addr | (self.or_mask as usize)) & (!self.float) as usize;

                for mask in FloatIterator::new(self.float) {
                    self.memory.insert(addr | mask as usize, *value);
                }
            }
        }
    }
}

struct FloatIterator {
    positions: Vec<u8>,
    current: usize,
    max: usize,
}

impl Iterator for FloatIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }

        let mut item = 0;

        for i in 0..self.positions.len() {
            if (self.current >> i) & 1 == 1 {
                item |= 1 << self.positions[i]
            }
        }
        self.current += 1;

        Some(item)
    }
}

impl FloatIterator {
    pub fn new(digits: u64) -> Self {
        let positions: Vec<u8> = (0..36)
            .into_iter()
            .filter(|p| (digits >> p) & 1 == 1)
            .collect();

        let max = (2 as usize).pow(positions.len() as u32) - 1;

        Self {
            positions,
            max,
            current: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string("data/day14.txt")?;
    let code = parse_code(&raw)?;

    let mut solver = Solver::new();
    solver.run(&code);
    let task_a = solver.sum();

    let mut solver = Solver::new();
    solver.run_v2(&code);
    let task_b = solver.sum();

    dbg!(task_a);
    dbg!(task_b);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_mask() {
        assert_eq!(
            Command::try_from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap(),
            Command::SetMask {
                and: 0b1111_11111111_11111111_11111111_11111101,
                or: 0b1000000,
                float: 0b1111_11111111_11111111_11111111_10111101,
            }
        );
    }

    #[test]
    fn test_parse_mem() {
        assert_eq!(
            Command::try_from("mem[8] = 11").unwrap(),
            Command::SetMem { addr: 8, value: 11 }
        );
    }

    #[test]
    fn test_float_iter() {
        assert_eq!(vec![0, 1], FloatIterator::new(0b1).collect::<Vec<_>>());
        assert_eq!(vec![0, 2], FloatIterator::new(0b10).collect::<Vec<_>>());
        assert_eq!(
            vec![0, 2, 8, 10],
            FloatIterator::new(0b1010).collect::<Vec<_>>()
        );
    }
}
