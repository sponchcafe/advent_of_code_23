use crate::util::load_lines;
use std::iter::Iterator;
use std::str::FromStr;

pub fn puzzle_1() -> u32 {
    load_lines("1/input.txt")
        .map(|l| {
            l.unwrap()
                .parse::<Calibration>()
                .expect("input is parseable")
        })
        .map(|c| c.number())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Calibration(u32, u32);

#[derive(Debug)]
struct CalibrationParseError {}

impl Calibration {
    fn number(self) -> u32 {
        self.0 * 10 + self.1
    }
}

impl FromStr for Calibration {
    type Err = CalibrationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars().filter(|c| c.is_digit(10)).map(|c| {
            let mut buf = [0u8; 4];
            let s = c.encode_utf8(&mut buf);
            str::parse::<u32>(s).expect("single digits are parseable")
        });
        let first = iter.next().ok_or(CalibrationParseError {})?;
        let last = iter.last().unwrap_or(first);
        Ok(Calibration(first, last))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[(&str, (u32, u32))] = &[
        ("1abc2", (1, 2)),
        ("pqr3stu8vwx", (3, 8)),
        ("a1b2c3d4e5f", (1, 5)),
        ("treb7uchet", (7, 7)),
    ];

    #[test]
    fn test_parse_calibration_numbers() {
        for (input, expected) in EXAMPLE.iter() {
            let cal = input
                .parse::<Calibration>()
                .expect("examples should be valid");
            assert_eq!(Calibration(expected.0, expected.1), cal);
        }
    }
}
