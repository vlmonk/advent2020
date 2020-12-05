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
}
