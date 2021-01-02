mod lexer;
mod parser;

use parser::{parse_a, parse_b};
use std::fs;

fn main() {
    let raw = fs::read_to_string("data/day18.txt").unwrap();
    let task_a: usize = raw.lines().map(|line| parse_a(line).value()).sum();
    let task_b: usize = raw.lines().map(|line| parse_b(line).value()).sum();

    println!("Task A: {}", task_a);
    println!("Task B: {}", task_b);
}
