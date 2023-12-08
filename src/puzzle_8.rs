use crate::util::load_lines;
use anyhow::Error;
use num::integer::Integer;
use std::collections::HashMap;
use std::iter::repeat;
use std::str::FromStr;

pub fn puzzle_8_1() -> u64 {
    let (directions, map) = get_input();
    map.path_length(&directions) as u64
}

pub fn puzzle_8_2() -> u64 {
    let (directions, map) = get_input();
    map.multi_path_length(&directions) as u64
}

fn get_input() -> (Directions, Map) {
    let mut lines = load_lines("8/input.txt");
    let directions = lines
        .next()
        .expect("read failed")
        .expect("no line")
        .parse::<Directions>()
        .unwrap();

    _ = lines.next();

    let paths = lines
        .map(|l| str::parse::<Path>(&l.expect("no line")))
        .collect::<Result<Vec<Path>, _>>()
        .unwrap();

    (directions, Map::new(&paths[..]))
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Directions {
    directions: Vec<Direction>,
}

impl FromStr for Directions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions = s
            .trim()
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => Err(Self::Err::msg("invalid direction")),
            })
            .collect::<Result<Vec<Direction>, _>>();
        Ok(Directions {
            directions: directions?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    from: String,
    left: String,
    right: String,
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Path, Self::Err> {
        let mut it = s.trim().split("=");
        let from = it.next().ok_or(Error::msg("no from"))?.trim().to_string();
        let to = it
            .next()
            .ok_or(Error::msg("no_directions"))?
            .trim()
            .replace(&['(', ')', ','], "");
        let mut to = to.split(char::is_whitespace);

        Ok(Path {
            from,
            left: to.next().ok_or(Error::msg("no left"))?.to_string(),
            right: to.next().ok_or(Error::msg("no right"))?.to_string(),
        })
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<String, (String, String)>,
}

impl Map {
    fn new(paths: &[Path]) -> Map {
        Map {
            map: paths
                .iter()
                .map(|path| (path.from.clone(), (path.left.clone(), path.right.clone())))
                .collect(),
        }
    }

    fn path_length(&self, directions: &Directions) -> usize {
        let start: String = String::from("AAA");
        let end: String = String::from("ZZZ");
        let mut cur = start;
        let mut steps = 0;
        for dir in directions.directions.iter().cycle() {
            if cur == end {
                break;
            };
            cur = match dir {
                Direction::Left => self.map.get(&cur).expect("dead path").0.clone(),
                Direction::Right => self.map.get(&cur).expect("dead path").1.clone(),
            };
            steps += 1;
        }
        steps
    }

    fn multi_path_length(&self, directions: &Directions) -> usize {
        let start: Vec<String> = self
            .map
            .keys()
            .filter(|&k| k.ends_with("A"))
            .cloned()
            .collect();
        let mut cur = start.clone();
        let mut steps = Vec::with_capacity(cur.len());
        steps.extend(repeat(0).take(cur.len()));
        let mut step = 0;
        for dir in directions.directions.iter().cycle() {
            cur.iter()
                .enumerate()
                .filter(|(_, k)| k.ends_with("Z"))
                .for_each(|(i, _)| {
                    steps[i] = step;
                });

            if steps.iter().all(|i| *i > 0) {
                break;
            }
            match dir {
                Direction::Left => cur.iter_mut().for_each(|k| {
                    *k = self.map.get(k).expect("dead path").0.clone();
                }),
                Direction::Right => cur.iter_mut().for_each(|k| {
                    *k = self.map.get(k).expect("dead path").1.clone();
                }),
            };
            step += 1;
        }
        steps.iter().fold(1, |acc, cur| acc.lcm(cur))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_directions() {
        use Direction::*;
        let directions = vec![Left, Left, Right, Left, Left, Right];
        assert_eq!(
            Directions { directions },
            "LLRLLR".parse::<Directions>().unwrap()
        );
    }

    #[test]
    fn test_parse_path() {
        let path = "AAA = (BBB, CCC)";
        let expected = Path {
            from: "AAA".into(),
            left: "BBB".into(),
            right: "CCC".into(),
        };

        assert_eq!(expected, path.parse::<Path>().unwrap());
    }

    #[test]
    fn test_example() {
        const EXAMPLE: &str = "
            LLR
    
            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ";

        let directions = EXAMPLE
            .trim()
            .lines()
            .next()
            .unwrap()
            .parse::<Directions>()
            .unwrap();
        let paths = EXAMPLE
            .trim()
            .lines()
            .skip(2)
            .map(str::parse::<Path>)
            .collect::<Result<Vec<Path>, _>>()
            .unwrap();

        let map = Map::new(&paths[..]);
        assert_eq!(6, map.path_length(&directions));
    }
}
