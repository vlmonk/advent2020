use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Bus {
    Active(usize),
    Canceled,
}

impl TryFrom<&str> for Bus {
    type Error = Box<dyn Error>;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input == "x" {
            return Ok(Bus::Canceled);
        }

        if let Ok(v) = input.parse::<usize>() {
            Ok(Bus::Active(v))
        } else {
            Err("Invalid input".into())
        }
    }
}

struct Input {
    start_time: usize,
    buses: Vec<Bus>,
}

impl TryFrom<&str> for Input {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut lines = input.lines();
        let start_time = lines
            .next()
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or("Invalid input")?;

        let buses = lines.next().ok_or("Invalid input".into()).and_then(|l| {
            l.split(',')
                .map(Bus::try_from)
                .collect::<Result<Vec<_>, _>>()
        })?;
        dbg!(buses);
        dbg!(start_time);

        Ok(Self {
            start_time,
            buses: vec![],
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string("data/day13.txt")?;
    let input: Input = raw[..].try_into()?;
    Ok(())
}
