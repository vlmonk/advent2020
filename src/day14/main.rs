use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug, PartialEq)]
enum Command {
    SetMask { and: u64, or: u64 },
    SetMem { addr: usize, value: u64 },
}

fn parse_mask(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
    if input.len() != 36 {
        return Err("invalid input len".into());
    }

    let mut and: u64 = 0;
    let mut or: u64 = 0;

    for (index, c) in input.chars().enumerate() {
        let offset = 35 - index;

        match c {
            'X' => and |= 1 << offset,
            '0' => {}
            '1' => {
                and |= 1 << offset;
                or |= 1 << offset;
            }
            _ => return Err("invalid input".into()),
        }
    }

    Ok((and, or))
}

impl TryFrom<&str> for Command {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if &input[0..7] == "mask = " {
            let (and, or) = parse_mask(&input[7..])?;
            Ok(Command::SetMask { and, or })
        } else {
            Err("Invalid command".into())
        }
    }
}

fn main() {
    println!("Placeholder!");
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
                or: 0b1000000
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
}
