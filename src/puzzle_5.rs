use crate::util::load_lines;
use anyhow::{Error, Result};
use std::{collections::BTreeMap, str::FromStr};

pub fn puzzle_5_1() -> u64 {
    let mut lines = load_lines("5/input.txt").map(Result::unwrap);
    let seeds = parse_seeds(&mut lines).expect("seeds");
    lines.next();
    let maps = parse_maps(&mut lines);

    seeds
        .into_iter()
        .map(|seed| remap_all(seed, &maps))
        .fold(u64::MAX, |min, val| u64::min(min, val))
}

fn parse_seeds(lines: &mut impl Iterator<Item = String>) -> Result<Vec<u64>> {
    let mut seedlines = lines.take_while(|l| !l.is_empty());
    seedlines
        .next()
        .ok_or(Error::msg("no seed line"))?
        .split(":")
        .last()
        .ok_or(Error::msg("no seeds"))?
        .trim()
        .split(" ")
        .map(str::trim)
        .map(|s| s.parse::<u64>().or(Err(Error::msg("invalid seed"))))
        .collect()
}

fn parse_maps(lines: &mut impl Iterator<Item = String>) -> Vec<AgriMap> {
    let mut maps = vec![];
    loop {
        match parse_map(lines) {
            Ok(map) => maps.push(map),
            _ => break,
        }
    }
    maps
}

fn parse_map(lines: &mut impl Iterator<Item = String>) -> Result<AgriMap> {
    let mut maplines = lines.take_while(|l| !l.is_empty());
    let name = maplines.next().ok_or(Error::msg("no name"))?;
    println!("{}", name);
    let mut map = AgriMap::new();
    for remap in maplines.map(|l| l.trim().parse::<Remap>()) {
        map.push(remap?);
    }
    Ok(map)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range(u64, u64);

impl Range {
    fn contains(&self, val: u64) -> bool {
        self.0 <= val && val < self.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Remap {
    dest: u64,
    source: u64,
    range: u64,
}

#[derive(Debug, PartialEq)]
struct AgriMap {
    remaps: BTreeMap<Range, Range>,
}

impl AgriMap {
    fn new() -> Self {
        AgriMap {
            remaps: BTreeMap::<Range, Range>::new(),
        }
    }
    fn get(&self, val: u64) -> u64 {
        for remap in self.remaps.iter() {
            if remap.0.contains(val) {
                return val + remap.1 .0 - remap.0 .0;
            }
        }
        val
    }
    fn push(&mut self, remap: Remap) {
        self.remaps.insert(
            Range(remap.source, remap.source + remap.range),
            Range(remap.dest, remap.dest + remap.range),
        );
    }
}

impl FromStr for Remap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" ").map(str::trim).map(str::parse::<u64>);
        let dest = it
            .next()
            .ok_or(Error::msg("no dest"))?
            .or(Err(Error::msg("invalid dest")))?;
        let source = it
            .next()
            .ok_or(Error::msg("no source"))?
            .or(Err(Error::msg("invalid source")))?;
        let range = it
            .next()
            .ok_or(Error::msg("no range"))?
            .or(Err(Error::msg("invalid range")))?;
        Ok(Remap {
            dest,
            source,
            range,
        })
    }
}

fn remap_all(val: u64, maps: &[AgriMap]) -> u64 {
    maps.iter()
        .fold(val, |intermediate, map| map.get(intermediate))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_direct_index() {
        let map = AgriMap::new();
        for i in 0..100 {
            assert_eq!(map.get(i), i);
        }
    }

    #[test]
    fn test_remapped_index() {
        let mut map = AgriMap::new();
        map.push(Remap {
            dest: 60,
            source: 50,
            range: 10,
        });
        for i in 0..50 {
            assert_eq!(map.get(i), i);
        }
        for i in 50..60 {
            assert_eq!(map.get(i), i + 10);
        }
        for i in 70..100 {
            assert_eq!(map.get(i), i);
        }
    }

    #[test]
    fn test_example() {
        let mut map = AgriMap::new();
        map.push(Remap {
            dest: 50,
            source: 98,
            range: 2,
        });
        map.push(Remap {
            dest: 52,
            source: 50,
            range: 48,
        });
        assert_eq!(map.get(79), 81);
        assert_eq!(map.get(14), 14);
        assert_eq!(map.get(55), 57);
        assert_eq!(map.get(13), 13);
        assert_eq!(map.get(99), 51);
        assert_eq!(map.get(98), 50);
    }

    #[test]
    fn test_multiple_maps() {
        let maps = vec![AgriMap::new(), AgriMap::new()];
        assert_eq!(42, remap_all(42, &maps));

        let mut map1 = AgriMap::new();
        let mut map2 = AgriMap::new();
        map1.push(Remap {
            dest: 50,
            source: 10,
            range: 2,
        });
        map2.push(Remap {
            dest: 152,
            source: 50,
            range: 5,
        });
        let maps = [map1, map2];
        assert_eq!(153, remap_all(11, &maps));
    }

    #[test]
    fn test_parse_map() {
        let input = "name\n10 20 3\n 35 45 5\n";
        let mut expected = AgriMap::new();
        expected.push(Remap {
            dest: 10,
            source: 20,
            range: 3,
        });
        expected.push(Remap {
            dest: 35,
            source: 45,
            range: 5,
        });
        let mut lines = input.split("\n").map(str::to_owned);
        assert_eq!(expected, parse_map(&mut lines).unwrap());
    }

    #[test]
    fn test_parse_maps() {
        let input = "one\n10 20 3\n 35 45 5\n\ntwo\n11 12 2\n";
        let mut lines = input.split("\n").map(str::to_owned);
        assert_eq!(2, parse_maps(&mut lines).len());
    }
}
