#[derive(Debug, PartialEq)]
pub enum Lex {
    Num(usize),
    Column,
    Char(char),
    Pipe,
}

pub struct LexerIter<'a> {
    input: &'a str,
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_space(c: char) -> bool {
    c == ' '
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = Lex;

    fn next(&mut self) -> Option<Self::Item> {
        while self.input.starts_with(is_space) {
            self.seek(1)
        }

        let mut iter = self.input.chars();

        match iter.nth(0) {
            Some(c) if is_digit(c) => {
                let first_non_digit = self
                    .input
                    .find(|c| !is_digit(c))
                    .unwrap_or(self.input.len());
                let value = self.input[0..first_non_digit].parse::<usize>().unwrap();
                self.seek(first_non_digit);

                Some(Lex::Num(value))
            }
            Some(c) if c == ':' => {
                self.seek(1);
                Some(Lex::Column)
            }
            Some(c) if c == '|' => {
                self.seek(1);
                Some(Lex::Pipe)
            }
            Some(c) if c == '"' => {
                let value = iter.nth(0);
                let close = iter.nth(0);

                self.seek(3);
                match (value, close) {
                    (Some(c), Some('"')) => Some(Lex::Char(c)),
                    _ => None,
                }
            }
            _ => None,
        }
        // None
    }
}

impl<'a> LexerIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn seek(&mut self, n: usize) {
        self.input = &self.input[n..];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num() {
        let input = "109";
        let result = LexerIter::new(&input).collect::<Vec<_>>();
        assert_eq!(result, vec![Lex::Num(109)]);
    }

    #[test]
    fn test_column() {
        let input = ":";
        let result = LexerIter::new(&input).collect::<Vec<_>>();
        assert_eq!(result, vec![Lex::Column]);
    }

    #[test]
    fn test_whitespace() {
        let input = "109  : 19";
        let result = LexerIter::new(&input).collect::<Vec<_>>();
        assert_eq!(result, vec![Lex::Num(109), Lex::Column, Lex::Num(19)]);
    }

    #[test]
    fn test_char() {
        let input = "\"a\"  \"b\"";
        let result = LexerIter::new(&input).collect::<Vec<_>>();
        assert_eq!(result, vec![Lex::Char('a'), Lex::Char('b')]);
    }
}
