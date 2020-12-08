use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;
type Error = Box<dyn std::error::Error>;

#[derive(Debug, PartialEq)]
enum Insruction {
    Noop,
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

        let captures = RE.captures(input).ok_or_else(|| "Invalid input")?;
        dbg!(&captures);

        let op = captures.get(1).map(|c| c.as_str()).ok_or_else(err)?;

        dbg!(op);

        let value = captures
            .get(2)
            .map(|c| c.as_str())
            .and_then(|c| c.parse::<i32>().ok())
            .ok_or_else(err)?;

        dbg!(value);

        match (op, value) {
            ("nop", _) => Ok(Insruction::Noop),
            ("acc", v) => Ok(Insruction::Acc(v)),
            ("jmp", v) => Ok(Insruction::Jmp(v)),
            (_, _) => Err(err().into()),
        }
    }
}

fn main() {
    println!("Hello");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_ok() {
        assert_eq!(Insruction::try_from("nop +0").unwrap(), Insruction::Noop);
        assert_eq!(Insruction::try_from("acc +1").unwrap(), Insruction::Acc(1));
        assert_eq!(
            Insruction::try_from("jpp -100").unwrap(),
            Insruction::Jmp(-100)
        );
    }
}
