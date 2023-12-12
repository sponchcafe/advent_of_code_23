#![allow(unused)]
use crate::util::load_lines;
use anyhow::Error;
use std::str::FromStr;

pub fn puzzle_12_1() -> u64 {
    parse_input()
        .iter()
        .map(|s| s.possibilities())
        .sum::<usize>() as u64
}

fn parse_input() -> Vec<Stage> {
    load_lines("12/input.txt")
        .map(|l| {
            let line = l.expect("line");
            let mut it = line.split(char::is_whitespace);
            let springs = it.next().expect("springs");
            let stretches = it.next().expect("stretches");
            let springs = springs.parse::<Springs>().expect("valid springs");
            let stretches = stretches
                .split(',')
                .map(|s| s.parse::<usize>().expect("valid stretch"))
                .collect::<Vec<usize>>();
            Stage { springs, stretches }
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Spring {
    Op,
    Dmg,
    Tbd,
    End,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Springs(Vec<Spring>);

impl FromStr for Springs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let springs = s
            .chars()
            .map(|c| match c {
                '.' => Ok(Spring::Op),
                '#' => Ok(Spring::Dmg),
                '?' => Ok(Spring::Tbd),
                _ => Err(Error::msg("unknown spring state")),
            })
            .collect::<Result<Vec<Spring>, _>>()?;
        Ok(Springs(springs))
    }
}

impl Springs {
    fn complete_groups(&self) -> Vec<usize> {
        self.0
            .iter()
            .chain(&Some(Spring::End)) // Insert end marker -> Options are iterable once
            .scan(0usize, |state, s| match s {
                Spring::Tbd => None,
                Spring::Dmg => {
                    *state += 1;
                    Some(0)
                }
                Spring::Op | Spring::End => {
                    let group = *state;
                    *state = 0;
                    Some(group)
                }
            })
            .filter(|g| *g > 0)
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Stage {
    springs: Springs,
    stretches: Vec<usize>,
}

impl Stage {
    fn repr(&self) -> String {
        self.springs
            .0
            .iter()
            .map(|s| match s {
                Spring::Op => '.',
                Spring::Dmg => '#',
                Spring::Tbd => '?',
                _ => 'X',
            })
            .collect()
    }

    fn done(&self) -> bool {
        self.springs.0.iter().find(|s| s == &&Spring::Tbd).is_none()
    }

    fn possible(&self) -> bool {
        if self.done() {
            self.stretches == self.springs.complete_groups()
        } else {
            possible(&self.stretches, &self.springs.complete_groups())
        }
    }

    fn possibilities(&self) -> usize {
        if let Some((left, right)) = self.split() {
            if !left.possible() {
                return right.possibilities();
            } else if !right.possible() {
                return left.possibilities();
            } else {
                return left.possibilities() + right.possibilities();
            }
        } else {
            if self.possible() {
                return 1;
            } else {
                return 0;
            }
        }
    }

    fn first_tbd(&mut self) -> Option<&mut Spring> {
        self.springs.0.iter_mut().find(|s| **s == Spring::Tbd)
    }

    fn split(&self) -> Option<(Stage, Stage)> {
        let mut left = self.clone();
        let mut right = self.clone();
        *(left.first_tbd()?) = Spring::Op;
        *(right.first_tbd()?) = Spring::Dmg;
        Some((left, right))
    }
}

fn possible(stretches: &[usize], groups: &[usize]) -> bool {
    if groups.len() > stretches.len() {
        false
    } else {
        stretches.iter().zip(groups).all(|(s, g)| g == s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Spring::*;

    #[test]
    fn test_possible() {
        assert!(possible(&[1, 1, 3], &[]));
        assert!(possible(&[1, 1, 3], &[1]));
        assert!(possible(&[1, 1, 3], &[1, 1]));
        assert!(possible(&[1, 1, 3], &[1, 1, 3]));

        assert!(!possible(&[1, 1, 3], &[1, 1, 2]));
        assert!(!possible(&[1, 1, 3], &[1, 1, 3, 1]));
        assert!(!possible(&[1, 1, 3], &[2]));
        assert!(!possible(&[1, 1, 3], &[1, 1, 4]));
    }

    #[test]
    fn test_possible_stages() {
        let stage = Stage {
            springs: Springs(vec![Tbd, Tbd, Tbd, Op, Dmg, Dmg, Dmg]),
            stretches: vec![1, 1, 3],
        };
        assert!(stage.possible());

        let stage = Stage {
            springs: Springs(vec![Op, Op, Dmg, Op, Dmg, Dmg, Dmg]),
            stretches: vec![1, 1, 3],
        };
        assert!(!stage.possible());

        let stage = Stage {
            springs: Springs(vec![Dmg, Op, Dmg, Op, Dmg, Dmg, Dmg]),
            stretches: vec![1, 1, 3],
        };
        assert!(stage.possible());

        let stage = Stage {
            springs: Springs(vec![Dmg, Dmg, Op, Tbd, Tbd, Tbd]),
            stretches: vec![1, 1, 3],
        };
        assert!(!stage.possible());

        let stage = Stage {
            springs: Springs(vec![Dmg, Dmg, Op, Dmg, Dmg]),
            stretches: vec![2, 2, 1],
        };
        assert!(!stage.possible());
    }

    #[test]
    fn test_split() {
        let stage = Stage {
            springs: Springs(vec![Tbd, Tbd, Tbd, Op, Dmg, Dmg, Dmg]),
            stretches: vec![1, 1, 3],
        };
        let left = Stage {
            springs: Springs(vec![Op, Tbd, Tbd, Op, Dmg, Dmg, Dmg]),
            stretches: vec![1, 1, 3],
        };
        let right = Stage {
            springs: Springs(vec![Dmg, Tbd, Tbd, Op, Dmg, Dmg, Dmg]),
            stretches: vec![1, 1, 3],
        };

        assert_eq!(Some((left, right)), stage.split());
    }

    #[test]
    fn test_groups() {
        let empty = Vec::<usize>::new();
        assert_eq!(
            empty,
            Springs(vec![Tbd, Tbd, Op, Op, Dmg, Tbd, Dmg]).complete_groups()
        );

        assert_eq!(
            empty,
            Springs(vec![Dmg, Tbd, Op, Op, Dmg, Tbd, Dmg]).complete_groups()
        );

        assert_eq!(
            vec![2],
            Springs(vec![Dmg, Dmg, Op, Op, Dmg, Tbd, Dmg]).complete_groups()
        );

        assert_eq!(
            vec![2, 1, 1],
            Springs(vec![Dmg, Dmg, Op, Op, Dmg, Op, Dmg]).complete_groups()
        );
    }

    #[test]
    fn test_possibilities() {
        let stage = Stage {
            springs: Springs(vec![Tbd, Tbd, Tbd, Op, Dmg, Dmg, Dmg]),
            stretches: vec![1, 1, 3],
        };
        assert_eq!(1, stage.possibilities());

        let stage = Stage {
            springs: Springs(vec![
                Op, Tbd, Tbd, Op, Op, Tbd, Tbd, Op, Op, Op, Tbd, Dmg, Dmg, Op,
            ]),
            stretches: vec![1, 1, 3],
        };
        assert_eq!(4, stage.possibilities());

        let stage = Stage {
            springs: Springs(vec![
                Tbd, Dmg, Tbd, Dmg, Tbd, Dmg, Tbd, Dmg, Tbd, Dmg, Tbd, Dmg, Tbd, Dmg, Tbd,
            ]),
            stretches: vec![1, 3, 1, 6],
        };
        assert_eq!(1, stage.possibilities());

        let stage = Stage {
            springs: Springs(vec![
                Tbd, Tbd, Tbd, Tbd, Op, Dmg, Op, Op, Op, Dmg, Op, Op, Op,
            ]),
            stretches: vec![4, 1, 1],
        };
        assert_eq!(1, stage.possibilities());

        let stage = Stage {
            springs: Springs(vec![
                Tbd, Tbd, Tbd, Tbd, Op, Dmg, Dmg, Dmg, Dmg, Dmg, Dmg, Op, Op, Dmg, Dmg, Dmg, Dmg,
                Dmg, Op,
            ]),
            stretches: vec![1, 6, 5],
        };
        assert_eq!(4, stage.possibilities());

        let stage = Stage {
            springs: Springs(vec![
                Tbd, Dmg, Dmg, Dmg, Tbd, Tbd, Tbd, Tbd, Tbd, Tbd, Tbd, Tbd,
            ]),
            stretches: vec![3, 2, 1],
        };
        assert_eq!(10, stage.possibilities());
    }
}
