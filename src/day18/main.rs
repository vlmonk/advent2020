mod lexer;
mod parser;

use parser::parse;

fn main() {
    let input = "1 + 2 + 3";
    let expr = parse(input);

    println!("E: {}", expr);
}
