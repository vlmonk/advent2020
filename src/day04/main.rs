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
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
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
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
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
            "eyr" => self.eyr = Some(value),
            "hgt" => self.hgt = Some(value),
            "hcl" => self.hcl = Some(value),
            "ecl" => self.ecl = Some(value),
            "pid" => self.pid = Some(value),
            "cid" => self.cid = Some(value),
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
