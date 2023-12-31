#![allow(unused)]

use crate::util::load_file;
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::{anychar, one_of, u32};
use nom::multi::many1_count;
use nom::{IResult, Offset};
use std::collections::{BTreeMap, BTreeSet};
use std::isize;

type Schematic = BTreeMap<Position, SchematicItem>;

pub fn puzzle_3_1() -> u32 {
    let input = load_file("3/input.txt");
    let width = input.find("\n").expect("at least one line");
    let schematic = schematic(&input, width);
    schematic
        .iter()
        .filter_map(|(pos, item)| item.number().map(|num| (pos, num)))
        .filter_map(|(pos, serial)| {
            for ref h in pos.hull() {
                if let Some(SchematicItem::Symbol { symbol: _ }) = schematic.get(h) {
                    return Some(serial);
                }
            }
            None
        })
        .fold(0, |sum, serial| sum + serial)
}

pub fn puzzle_3_2() -> u32 {
    let input = load_file("3/input.txt");
    let width = input.find("\n").expect("at least one line");
    let schematic = schematic(&input, width);
    let numbers: Vec<(Position, u32)> = schematic
        .iter()
        .filter_map(|(pos, item)| item.number().map(|n| (pos.clone(), n)))
        .collect();
    let mut ratios = BTreeMap::<Position, Vec<u32>>::new();
    for (p, n) in numbers.iter() {
        for h in p.hull() {
            if let Some(item) = schematic.get(&h) {
                if item.gear().is_some() {
                    if let Some(v) = ratios.get_mut(&h) {
                        v.push(*n);
                    } else {
                        ratios.insert(h.clone(), vec![*n]);
                    }
                }
            }
        }
    }

    ratios
        .iter()
        .filter(|(p, v)| v.len() == 2)
        .map(|(_, v)| v.iter().fold(1, |a, b| a * b))
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    line: isize,
    column: isize,
    len: usize,
}

impl Position {
    fn hull(&self) -> BTreeSet<Position> {
        let mut hull = BTreeSet::<Position>::new();
        for l in self.line - 1..self.line + 1 + 1 {
            for c in self.column - 1..self.column + self.len as isize + 1 {
                if l == self.line && c >= self.column && c < self.column + self.len as isize {
                    continue;
                }
                hull.insert(Position {
                    line: l,
                    column: c,
                    len: 1, // TODO: This should be part of the serial
                });
            }
        }
        hull
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = (self.line, self.column);
        let b = (other.line, other.column);
        a.cmp(&b)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SchematicItem {
    Number { number: u32 },
    Symbol { symbol: char },
    Space,
}

impl SchematicItem {
    fn number(&self) -> Option<u32> {
        match self {
            SchematicItem::Number { number: n } => Some(*n),
            _ => None,
        }
    }
    fn symbol(&self) -> Option<char> {
        match self {
            SchematicItem::Symbol { symbol: s } => Some(*s),
            _ => None,
        }
    }
    fn gear(&self) -> Option<()> {
        match self {
            SchematicItem::Symbol { symbol: '*' } => Some(()),
            _ => None,
        }
    }
}

fn space(input: &str) -> IResult<&str, SchematicItem> {
    let (output, _) = many1_count(one_of(".\n"))(input)?;
    Ok((output, SchematicItem::Space))
}

fn serial(input: &str) -> IResult<&str, SchematicItem> {
    let (output, number) = u32(input)?;
    Ok((output, SchematicItem::Number { number }))
}

fn symbol(input: &str) -> IResult<&str, SchematicItem> {
    let (output, c) = anychar(input)?;
    Ok((output, SchematicItem::Symbol { symbol: c }))
}

fn schematic(input: &str, line_width: usize) -> Schematic {
    let mut begin = input;
    let line_width = line_width + 1; // Account for newlines
    let mut schematic_map = Schematic::new();
    loop {
        match alt((serial, space, symbol))(begin) {
            Ok((end, item)) => {
                let pos = input.offset(begin) as isize;
                let len = begin.offset(end);
                if let SchematicItem::Space = item {
                    // Ignoring spaces
                } else {
                    let line = pos / (line_width) as isize;
                    let column = pos % (line_width) as isize;
                    let position = Position { line, column, len };
                    schematic_map.insert(position, item);
                }
                begin = end;
            }
            Err(_) => break,
        };
    }
    schematic_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_space() {
        assert!(space("").is_err());
        assert_eq!(Ok(("", SchematicItem::Space)), space("."));
        assert_eq!(Ok(("", SchematicItem::Space)), space("\n"));
        assert_eq!(Ok(("", SchematicItem::Space)), space("..."));
        assert_eq!(Ok(("", SchematicItem::Space)), space("...\n..."));

        let input = "...123";
        let (input, parsed) = space(input).unwrap();
        assert_eq!(SchematicItem::Space, parsed);
        assert_eq!("123", input);
    }

    #[test]
    fn test_serial() {
        assert_eq!(
            Ok(("", SchematicItem::Number { number: 123 })),
            serial("123")
        );
        assert_eq!(
            Ok(("...", SchematicItem::Number { number: 0 })),
            serial("0...")
        );
    }

    #[test]
    fn test_symbol() {
        assert_eq!(Ok(("", SchematicItem::Symbol { symbol: '?' })), symbol("?"));
        assert_eq!(
            Ok(("..", SchematicItem::Symbol { symbol: '*' })),
            symbol("*..")
        );
    }

    #[test]
    fn test_parse_empty_schematic() {
        assert_eq!(Schematic::new(), schematic("", 0));
        assert_eq!(Schematic::new(), schematic("..........", 10));
        assert_eq!(Schematic::new(), schematic(".....\n.....", 5));
    }

    #[test]
    fn test_position() {
        let input = "+....\n.....\n..+..\n.....\n....+\n";
        let mut expected = Schematic::new();
        expected.insert(
            Position {
                line: 0,
                column: 0,
                len: 1,
            },
            SchematicItem::Symbol { symbol: '+' },
        );
        expected.insert(
            Position {
                line: 2,
                column: 2,
                len: 1,
            },
            SchematicItem::Symbol { symbol: '+' },
        );
        expected.insert(
            Position {
                line: 4,
                column: 4,
                len: 1,
            },
            SchematicItem::Symbol { symbol: '+' },
        );
        assert_eq!(expected, schematic(input, 5));
    }

    #[test]
    fn test_parse_full_schematic() {
        let input = "..6..\n.123*\n.....\n.+.4.\n99.$.";
        let mut expected = Schematic::new();
        expected.insert(
            Position {
                line: 0,
                column: 2,
                len: 1,
            },
            SchematicItem::Number { number: 6 },
        );
        expected.insert(
            Position {
                line: 1,
                column: 1,
                len: 3,
            },
            SchematicItem::Number { number: 123 },
        );
        expected.insert(
            Position {
                line: 1,
                column: 4,
                len: 1,
            },
            SchematicItem::Symbol { symbol: '*' },
        );
        expected.insert(
            Position {
                line: 3,
                column: 1,
                len: 1,
            },
            SchematicItem::Symbol { symbol: '+' },
        );
        expected.insert(
            Position {
                line: 3,
                column: 3,
                len: 1,
            },
            SchematicItem::Number { number: 4 },
        );
        expected.insert(
            Position {
                line: 4,
                column: 0,
                len: 2,
            },
            SchematicItem::Number { number: 99 },
        );
        expected.insert(
            Position {
                line: 4,
                column: 3,
                len: 1,
            },
            SchematicItem::Symbol { symbol: '$' },
        );
        assert_eq!(expected, schematic(input, 5));
    }

    #[test]
    fn test_position_hull_size() {
        let pos = Position {
            line: 0,
            column: 0,
            len: 1,
        };
        assert_eq!(8, pos.hull().len());

        let pos = Position {
            line: 0,
            column: 0,
            len: 3,
        };
        assert_eq!(12, pos.hull().len());
    }

    #[test]
    fn test_position_hull() {
        let pos = Position {
            line: 0,
            column: 0,
            len: 1,
        };

        let points = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let positions = points
            .iter()
            .map(|p| Position {
                line: p.0,
                column: p.1,
                len: 1,
            })
            .collect::<BTreeSet<Position>>();

        assert_eq!(positions, pos.hull());
    }

    #[test]
    fn test_get_number() {
        assert_eq!(Some(123), SchematicItem::Number { number: 123 }.number());
    }
}
