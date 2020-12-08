use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::convert::TryFrom;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, PartialEq)]
enum Insruction {
    Noop(i32),
    Acc(i32),
    Jmp(i32),
}

impl TryFrom<&str> for Insruction {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+)\s+([+-]\d+)$").unwrap();
        }

        let err = || format!("invald input: {}", input);

        let captures = RE.captures(input).ok_or_else(err)?;
        let op = captures.get(1).map(|c| c.as_str()).ok_or_else(err)?;
        let value = captures
            .get(2)
            .map(|c| c.as_str())
            .and_then(|c| c.parse::<i32>().ok())
            .ok_or_else(err)?;

        match (op, value) {
            ("nop", v) => Ok(Insruction::Noop(v)),
            ("acc", v) => Ok(Insruction::Acc(v)),
            ("jmp", v) => Ok(Insruction::Jmp(v)),
            (_, _) => Err(err().into()),
        }
    }
}

type Programm = Vec<Insruction>;

fn parse(input: &str) -> Result<Programm, Error> {
    input.lines().map(Insruction::try_from).collect()
}

struct CPU {
    prog: Programm,
    ip: usize,
    acc: i32,
}

impl CPU {
    pub fn new(prog: Programm) -> Self {
        Self {
            prog,
            ip: 0,
            acc: 0,
        }
    }

    pub fn ip(&self) -> usize {
        self.ip
    }

    pub fn acc(&self) -> i32 {
        self.acc
    }

    pub fn step(&mut self) {
        dbg!(&self.ip);
        dbg!(&self.prog[self.ip]);
        match self.prog[self.ip] {
            Insruction::Noop(_) => {
                self.ip += 1;
            }
            Insruction::Acc(v) => {
                self.ip += 1;
                self.acc += v;
            }
            Insruction::Jmp(v) => {
                let next = self.ip as i32 + v;
                self.ip = next as usize;
            }
        }
    }
}

struct Solver {
    cpu: CPU,
}

impl Solver {
    pub fn new(prog: Programm) -> Self {
        Self {
            cpu: CPU::new(prog),
        }
    }

    pub fn solve(&mut self) -> i32 {
        let mut visited: HashSet<usize> = HashSet::new();

        loop {
            self.cpu.step();
            let ip = self.cpu.ip();

            if visited.contains(&ip) {
                return self.cpu.acc();
            }

            visited.insert(ip);
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day08.txt").unwrap();
    let programm = parse(&data).unwrap();
    let value = Solver::new(programm).solve();
    dbg!(value);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_parse_ok() {
        assert_eq!("nop +0".try_into(), Ok(Insruction::Noop(0)));
        assert_eq!("acc +1".try_into(), Ok(Insruction::Acc(1)));
        assert_eq!(("jmp -100").try_into(), Ok(Insruction::Jmp(-100)));
    }
}
