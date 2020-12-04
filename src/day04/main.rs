use std::error::Error;
use std::fs;

fn parse_byr(input: &str) -> Option<u32> {
    match input.parse::<u32>() {
        Ok(value) if value >= 1920 && value <= 2002 => Some(value),
        _ => None,
    }
}

fn parse_iyr(input: &str) -> Option<u32> {
    match input.parse::<u32>() {
        Ok(value) if value >= 1920 && value <= 2002 => Some(value),
        _ => None,
    }
}

#[derive(Default)]
struct Passprt<'a> {
    byr_raw: Option<&'a str>,
    byr: Option<u32>,
    iyr_raw: Option<&'a str>,
    iyr: Option<u32>,
    eyr_raw: Option<&'a str>,
    hgt_raw: Option<&'a str>,
    hcl_raw: Option<&'a str>,
    ecl_raw: Option<&'a str>,
    pid_raw: Option<&'a str>,
    cid_raw: Option<&'a str>,
}

impl<'a> Passprt<'a> {
    pub fn parse(input: &'a str) -> Result<Passprt<'a>, Box<dyn Error>> {
        let mut passport = Self::default();
        let items = input
            .split(|c| c == '\n' || c == ' ')
            .filter(|i| i.len() > 0);
        for item in items {
            let mut entry = item.split(':');
            let key = entry
                .next()
                .ok_or_else(|| format!("Key not found: {}", item))?;
            let value = entry
                .next()
                .ok_or_else(|| format!("Value not found: {}", item))?;

            passport.set(key, value)?;
        }

        Ok(passport)
    }

    pub fn valid(&self) -> bool {
        self.byr_raw.is_some()
            && self.iyr_raw.is_some()
            && self.eyr_raw.is_some()
            && self.hgt_raw.is_some()
            && self.hcl_raw.is_some()
            && self.ecl_raw.is_some()
            && self.pid_raw.is_some()
    }

    fn set(&mut self, key: &str, value: &'a str) -> Result<(), Box<dyn Error>> {
        match key {
            "byr" => {
                self.byr_raw = Some(value);
                self.byr = parse_byr(value);
            }
            "iyr" => {
                self.iyr_raw = Some(value);
                self.iyr = parse_iyr(value);
            }
            "eyr" => self.eyr_raw = Some(value),
            "hgt" => self.hgt_raw = Some(value),
            "hcl" => self.hcl_raw = Some(value),
            "ecl" => self.ecl_raw = Some(value),
            "pid" => self.pid_raw = Some(value),
            "cid" => self.cid_raw = Some(value),
            _ => return Err(format!("Invalid field: {}:{}", key, value).into()),
        };

        Ok(())
    }
}

fn main() {
    let data = fs::read_to_string("data/day04.txt").unwrap();
    let chunks = data.split("\n\n");
    let passport = chunks
        .map(|chunk| Passprt::parse(chunk))
        .collect::<Result<Vec<_>, _>>()
        .expect("bad input");
    let valid = passport.iter().filter(|p| p.valid()).count();
    dbg!(valid);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "byr:1928";
        let passport = Passprt::parse(&input).unwrap();
        assert_eq!(passport.byr_raw, Some("1928"));
        assert_eq!(passport.byr, Some(1928));
    }

    #[test]
    fn test_parse_fail() {
        let input = "byr:1928 foo:bar";
        let passport = Passprt::parse(&input);
        assert_eq!(passport.is_err(), true);
    }

    #[test]
    fn test_parse_invalid_year() {
        let input = "byr:1828";
        let passport = Passprt::parse(&input).unwrap();
        assert_eq!(passport.byr, None);
    }
}
