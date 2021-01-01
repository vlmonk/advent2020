mod lexer;
mod parser;

use parser::parse;
use std::fs;

fn main() {
    let raw = fs::read_to_string("data/day18.txt").unwrap();
    let task_a: usize = raw.lines().map(|line| parse(line).value()).sum();

    println!("Task A: {}", task_a);
}
