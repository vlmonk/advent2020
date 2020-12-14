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

fn parse_mem(input: &str) -> Result<(usize, u64), Box<dyn Error>> {
    let i_start = input.find('[').ok_or("invalid input")?;
    let i_end = input.find(']').ok_or("invalid input")?;
    let eq = input.find('=').ok_or("invalid input")?;

    let addr = input[i_start + 1..i_end].parse::<usize>()?;
    let value = input[eq + 2..].parse::<u64>()?;

    Ok((addr, value))
}

impl TryFrom<&str> for Command {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if &input[0..7] == "mask = " {
            let (and, or) = parse_mask(&input[7..])?;
            Ok(Command::SetMask { and, or })
        } else if &input[0..3] == "mem" {
            let (addr, value) = parse_mem(&input)?;
            Ok(Command::SetMem { addr, value })
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
