fn binary(input: &str, one: char, zero: char) -> Option<u32> {
    let size = input.len();
    let mut result: u32 = 0;

    for (i, c) in input.chars().enumerate() {
        let shift = size - 1 - i;
        match c {
            c if c == one => result |= 1 << shift,
            c if c == zero => {}
            _ => return None,
        }
    }

    Some(result)
}

#[derive(PartialEq, Debug)]
struct BoardPass {
    row: u32,
    column: u32,
}

impl BoardPass {
    fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }

    pub fn parse(input: &str) -> Option<Self> {
        let row = binary(&input[0..7], 'B', 'F')?;
        let column = binary(&input[7..10], 'R', 'L')?;

        Some(Self::new(row, column))
    }
}

fn main() {
    println!("test me");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary() {
        assert_eq!(binary("10", '1', '0'), Some(2));
        assert_eq!(binary("1*0", '1', '0'), None);
        assert_eq!(binary("FBFBBFF", 'B', 'F'), Some(44));
    }

    #[test]
    fn test_board_pass() {
        assert_eq!(
            BoardPass::parse("BFFFBBFRRR").unwrap(),
            BoardPass::new(70, 7)
        );
    }
}
