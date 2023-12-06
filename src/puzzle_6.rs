use crate::util::load_lines;
use anyhow::{Error, Result};

pub fn puzzle_6_1() -> u64 {
    let mut lines = load_lines("6/input.txt");
    let times: Vec<u64> = parse_line(&mut lines).expect("valid times");
    let records: Vec<u64> = parse_line(&mut lines).expect("valid records");
    times
        .into_iter()
        .zip(records)
        .map(|(t, r)| Race { time: t, record: r })
        .map(|r| ways_to_win(&r))
        .fold(1, |acc, val| acc * val)
}

pub fn puzzle_6_2() -> u64 {
    let mut lines = load_lines("6/input.txt");
    let time = parse_line_joined(&mut lines).expect("valid time");
    let record = parse_line_joined(&mut lines).expect("valid record");
    ways_to_win(&Race { time, record })
}

fn parse_line<E>(
    lines: &mut impl Iterator<Item = std::result::Result<String, E>>,
) -> Result<Vec<u64>>
where
    E: std::error::Error + Sync + Send + 'static,
{
    lines
        .next()
        .ok_or(Error::msg("no line"))??
        .split(":")
        .last()
        .ok_or(Error::msg("no delimited numbers"))?
        .trim()
        .split(char::is_whitespace)
        .filter(|s| !str::is_empty(s))
        .map(str::parse::<u64>)
        .collect::<Result<Vec<u64>, _>>()
        .map_err(Error::from)
}

fn parse_line_joined<E>(
    lines: &mut impl Iterator<Item = std::result::Result<String, E>>,
) -> Result<u64>
where
    E: std::error::Error + Sync + Send + 'static,
{
    let num: String = lines
        .next()
        .ok_or(Error::msg("no line"))??
        .split(":")
        .last()
        .ok_or(Error::msg("no numbers"))?
        .chars()
        .filter(|c| !char::is_whitespace(*c))
        .collect();
    Ok(num.parse::<u64>()?)
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    record: u64,
}

/// Intersection of winning function with record R
/// (T-x) * x = R
/// T*x - x^2 = R
///
/// Quadratic expansion
/// x = 1/2 T +/- sqrt(T^2/4 - R)
fn ways_to_win(race: &Race) -> u64 {
    let t: f64 = race.time as f64;
    let r: f64 = race.record as f64;
    let a = 0.5 * t + f64::sqrt(f64::powf(t, 2.0) / 4.0 - r);
    let b = 0.5 * t - f64::sqrt(f64::powf(t, 2.0) / 4.0 - r);
    let cnt = integers_between(a, b);
    cnt
}

fn integers_between(mut a: f64, mut b: f64) -> u64 {
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    if a.fract() == 0.0 {
        a += 1.0; // Correction if float is exact integer
    }
    ((a.ceil() as u64)..(b.ceil() as u64)).count() as u64
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_count_ways_to_win() {
        assert_eq!(
            0,
            ways_to_win(&Race {
                time: 7,
                record: 12
            })
        );
        assert_eq!(4, ways_to_win(&Race { time: 7, record: 9 }));
        assert_eq!(
            8,
            ways_to_win(&Race {
                time: 15,
                record: 40
            })
        );
        assert_eq!(
            9,
            ways_to_win(&Race {
                time: 30,
                record: 200
            })
        );
    }
}
