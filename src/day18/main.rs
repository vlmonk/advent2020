mod lexer;

use lexer::MathLexer;

fn main() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";
    let mut lexer = MathLexer::new(input);
    dbg!(lexer.next());
}
