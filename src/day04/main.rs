use advent2020::measure;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fs;

#[derive(PartialEq, Debug)]
enum Hgt {
    Cm(u32),
    In(u32),
}

fn parse_byr(input: &str) -> Option<u32> {
    match input.parse::<u32>() {
        Ok(value) if value >= 1920 && value <= 2002 => Some(value),
        _ => None,
    }
}

fn parse_iyr(input: &str) -> Option<u32> {
    match input.parse::<u32>() {
        Ok(value) if value >= 2010 && value <= 2020 => Some(value),
        _ => None,
    }
}

fn parse_eyr(input: &str) -> Option<u32> {
    match input.parse::<u32>() {
        Ok(value) if value >= 2020 && value <= 2030 => Some(value),
        _ => None,
    }
}

fn parse_hgt(input: &str) -> Option<Hgt> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    }

    let cap = RE.captures(input)?;
    let value = cap.get(1).map(|m| m.as_str())?;
    let kind = cap.get(2).map(|m| m.as_str())?;

    match value.parse::<u32>() {
        Ok(value) if kind == "in" && value >= 59 && value <= 76 => Some(Hgt::In(value)),
        Ok(value) if kind == "cm" && value >= 150 && value <= 193 => Some(Hgt::Cm(value)),
        _ => None,
    }
}

fn parse_hcl(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    }

    if RE.is_match(input) {
        Some(input)
    } else {
        None
    }
}

fn parse_ecl(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }

    if RE.is_match(input) {
        Some(input)
    } else {
        None
    }
}

fn parse_pid(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }

    if RE.is_match(input) {
        Some(input)
    } else {
        None
    }
}

#[derive(Default)]
struct Passprt<'a> {
    byr_raw: Option<&'a str>,
    byr: Option<u32>,
    iyr_raw: Option<&'a str>,
    iyr: Option<u32>,
    eyr_raw: Option<&'a str>,
    eyr: Option<u32>,
    hgt_raw: Option<&'a str>,
    hgt: Option<Hgt>,
    hcl_raw: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl_raw: Option<&'a str>,
    ecl: Option<&'a str>,
    pid_raw: Option<&'a str>,
    pid: Option<&'a str>,
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

    pub fn valid_part2(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
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
            "eyr" => {
                self.eyr_raw = Some(value);
                self.eyr = parse_eyr(value);
            }
            "hgt" => {
                self.hgt_raw = Some(value);
                self.hgt = parse_hgt(value);
            }
            "hcl" => {
                self.hcl_raw = Some(value);
                self.hcl = parse_hcl(value);
            }
            "ecl" => {
                self.ecl_raw = Some(value);
                self.ecl = parse_ecl(value);
            }
            "pid" => {
                self.pid_raw = Some(value);
                self.pid = parse_pid(value);
            }
            "cid" => self.cid_raw = Some(value),
            _ => return Err(format!("Invalid field: {}:{}", key, value).into()),
        };

        Ok(())
    }
}

fn main() {
    let ((task_a, task_b), elapsed) = measure(|| {
        let data = fs::read_to_string("data/day04.txt").unwrap();
        let chunks = data.split("\n\n");
        let passport = chunks
            .map(|chunk| Passprt::parse(chunk))
            .collect::<Result<Vec<_>, _>>()
            .expect("bad input");
        let task_a = passport.iter().filter(|p| p.valid()).count();
        let task_b = passport.iter().filter(|p| p.valid_part2()).count();
        (task_a, task_b)
    });

    println!(
        "task A: {}\ntask B: {}\nTotal time: {}μs ",
        task_a, task_b, elapsed
    );
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

    #[test]
    fn test_parse_hgt() {
        let input = "hgt:190cm";
        let passport = Passprt::parse(&input).unwrap();
        assert_eq!(passport.hgt, Some(Hgt::Cm(190)));
    }

    #[test]
    fn test_parse_hgt_invalid() {
        let input = "hgt:195cm";
        let passport = Passprt::parse(&input).unwrap();
        assert_eq!(passport.hgt, None);
    }
}
