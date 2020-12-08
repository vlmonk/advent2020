use advent2020::measure;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::mem;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, PartialEq, Clone)]
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

#[derive(PartialEq)]
enum StepResult {
    Ok,
    Stop,
}

struct CPU<'a> {
    prog: &'a Programm,
    ip: usize,
    acc: i32,
}

impl<'a> CPU<'a> {
    pub fn new(prog: &'a Programm) -> Self {
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

    pub fn step(&mut self) -> StepResult {
        match self.prog.get(self.ip) {
            Some(Insruction::Noop(_)) => {
                self.ip += 1;
                StepResult::Ok
            }
            Some(Insruction::Acc(v)) => {
                self.ip += 1;
                self.acc += v;
                StepResult::Ok
            }
            Some(Insruction::Jmp(v)) => {
                let next = self.ip as i32 + v;
                self.ip = next as usize;
                StepResult::Ok
            }
            None => StepResult::Stop,
        }
    }
}

struct ProgMutator {
    prog: Programm,
    next: usize,
}

impl Iterator for ProgMutator {
    type Item = Programm;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.prog.get(self.next) {
                Some(Insruction::Noop(v)) => {
                    let mut clone = self.prog.clone();
                    let mut replace = Insruction::Jmp(*v);
                    mem::swap(&mut clone[self.next], &mut replace);
                    self.next += 1;

                    return Some(clone);
                }
                Some(Insruction::Jmp(v)) => {
                    let mut clone = self.prog.clone();
                    let mut replace = Insruction::Noop(*v);
                    mem::swap(&mut clone[self.next], &mut replace);
                    self.next += 1;

                    return Some(clone);
                }
                Some(_) => {}
                _ => return None,
            }

            self.next += 1;
        }
    }
}

impl ProgMutator {
    pub fn new(prog: Programm) -> Self {
        Self { prog, next: 0 }
    }
}

enum RunResult {
    Loop,
    Stop,
}

struct Solver {}

impl Solver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn solve(&self, prog: Programm) -> Option<i32> {
        let mut cpu = CPU::new(&prog);
        match self.run_till_stop(&mut cpu) {
            RunResult::Loop => Some(cpu.acc()),
            _ => None,
        }
    }

    pub fn solve_b(&self, prog: Programm) -> Option<i32> {
        ProgMutator::new(prog).find_map(|prog| {
            let mut cpu = CPU::new(&prog);
            let result = self.run_till_stop(&mut cpu);
            match result {
                RunResult::Stop => Some(cpu.acc()),
                _ => None,
            }
        })
    }

    fn run_till_stop(&self, cpu: &mut CPU) -> RunResult {
        let mut visited = HashSet::new();

        loop {
            let step_result = cpu.step();
            if step_result == StepResult::Stop {
                return RunResult::Stop;
            }

            if visited.contains(&cpu.ip()) {
                return RunResult::Loop;
            }

            visited.insert(cpu.ip());
        }
    }
}

fn main() {
    let ((a, b), elapsed) = measure(|| {
        let data = std::fs::read_to_string("data/day08.txt").unwrap();
        let programm = parse(&data).unwrap();

        (
            Solver::new().solve(programm.clone()),
            Solver::new().solve_b(programm),
        )
    });

    match a {
        Some(value) => println!("Task A: {}", value),
        _ => println!("Task A: not found"),
    };

    match b {
        Some(value) => println!("Task B: {}", value),
        _ => println!("Task B: not found"),
    };
    println!("Total time: {}μs", elapsed);
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
