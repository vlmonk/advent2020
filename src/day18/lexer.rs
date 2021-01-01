use std::iter::Peekable;
use std::str::Chars;

fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}

trait Lexer<T>: Iterator<Item = T> {
    fn peek(&mut self) -> Option<T>;
}

#[derive(Debug, PartialEq)]
pub enum MathLexem {
    Num(usize),
    Add,
    Mul,
    Eol,
}

pub struct MathLexer<'a> {
    inner: MathLexerInner<'a>,
}

struct MathLexerInner<'a> {
    chars: Peekable<Chars<'a>>,
    eol: bool,
}

impl<'a> MathLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: MathLexerInner::new(input),
        }
    }
}

impl<'a> MathLexerInner<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            eol: false,
        }
    }
}

impl<'a> Iterator for MathLexer<'a> {
    type Item = MathLexem;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> Iterator for MathLexerInner<'a> {
    type Item = MathLexem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(' ') = self.chars.peek() {
                self.chars.next();
            } else {
                break;
            }
        }

        match self.chars.next() {
            Some(c) if is_number(c) => {
                let mut buf = vec![c];
                loop {
                    match self.chars.peek() {
                        Some(c) if is_number(*c) => {
                            buf.push(*c);
                            self.next();
                        }
                        _ => break,
                    }
                }
                let buf: String = buf.iter().collect();
                let value = buf.parse::<usize>().unwrap();

                Some(MathLexem::Num(value))
            }
            Some(c) if c == '+' => Some(MathLexem::Add),
            Some(c) if c == '*' => Some(MathLexem::Mul),
            None if !self.eol => {
                self.eol = true;
                Some(MathLexem::Eol)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "42";
        assert_eq!(
            MathLexer::new(input).collect::<Vec<_>>(),
            vec![MathLexem::Num(42), MathLexem::Eol]
        )
    }

    #[test]
    fn test_val() {
        let input = "2  + 3 * 1 ";
        assert_eq!(
            MathLexer::new(input).collect::<Vec<_>>(),
            vec![
                MathLexem::Num(2),
                MathLexem::Add,
                MathLexem::Num(3),
                MathLexem::Mul,
                MathLexem::Num(1),
                MathLexem::Eol
            ]
        )
    }
}
