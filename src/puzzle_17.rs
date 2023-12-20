use std::fmt::Display;
use std::{cmp::Reverse, iter::repeat_with};

use crate::util::{load_file, Grid};
pub fn puzzle_17_1() -> u64 {
    0
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct HeatField {
    heat_loss: u32,
    traversal: Option<TraversalData>,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct TraversalData {
    total_heat_loss: u32,
    straight_steps: u8,
    position: Position,
    entry: Direction,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[repr(usize)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Position {
    fn new(row: isize, col: isize) -> Option<Position> {
        if row < 0 || col < 0 {
            None
        } else {
            Some(Position {
                row: row as usize,
                col: col as usize,
            })
        }
    }
}

impl Display for HeatField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.heat_loss))
    }
}

impl From<char> for HeatField {
    fn from(value: char) -> Self {
        HeatField {
            heat_loss: value.to_digit(10).unwrap_or(0),
            traversal: None,
        }
    }
}

fn get_straight_steps(steps: u8, enter: Direction, exit: Direction) -> u8 {
    if enter == exit {
        steps + 1
    } else {
        0
    }
}

impl Grid<HeatField> {
    fn at(&self, pos: &Position) -> &HeatField {
        &self[pos.row][pos.col]
    }

    fn at_mut(&mut self, pos: &Position) -> &mut HeatField {
        &mut self[pos.row][pos.col]
    }

    fn best_path(&mut self, from: Position, to: Position) {
        let mut path_heap = std::collections::BinaryHeap::<Reverse<(u32, TraversalData)>>::new();
        path_heap.push(Reverse((
            0,
            TraversalData {
                total_heat_loss: 0,
                straight_steps: 0,
                position: from,
                entry: Direction::North,
            },
        )));

        loop {
            if let Some(Reverse((_, trv))) = path_heap.pop() {
                if trv.straight_steps > 2 {
                    continue;
                };
                self.at_mut(&trv.position).traversal = Some(trv.clone());
                for (dir, conn) in self.connections(&trv.position).into_iter() {
                    path_heap.push(Reverse((
                        trv.total_heat_loss + self.at(&conn).heat_loss,
                        TraversalData {
                            total_heat_loss: trv.total_heat_loss + self.at(&conn).heat_loss,
                            straight_steps: get_straight_steps(trv.straight_steps, trv.entry, dir),
                            position: conn,
                            entry: dir,
                        },
                    )));
                }
                if trv.position == to {
                    return;
                }
            } else {
                panic!("no path from {:?} to {:?}", from, to);
            }
        }
    }

    fn connections(&self, pos: &Position) -> Vec<(Direction, Position)> {
        let r = pos.row as isize;
        let c = pos.col as isize;
        let raw: [(Direction, (isize, isize)); 4] = [
            (Direction::North, (r + 1, c)),
            (Direction::East, (r, c - 1)),
            (Direction::South, (r - 1, c)),
            (Direction::West, (r, c + 1)),
        ];

        raw.into_iter()
            .filter(|(_, (r, c))| {
                *r >= 0 && *c >= 0 && (*r as usize) < self.shape.0 && (*c as usize) < self.shape.1
            })
            .map(|(d, (row, col))| {
                (
                    d,
                    Position {
                        row: row as usize,
                        col: col as usize,
                    },
                )
            })
            .filter(|(_, p)| self.at(p).traversal.is_none())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid() {
        let data = load_file("17/example.txt");
        let g = Grid::<HeatField>::from_table_data(&data);
        assert_eq!(2, g[2][4].heat_loss);
        assert_eq!(8, g[6][8].heat_loss);
    }

    #[test]
    fn test_traversal() {
        let data = load_file("17/example.txt");
        let mut g = Grid::<HeatField>::from_table_data(&data);
        let start = Position::new(0, 0).unwrap();
        let end = Position::new(12, 12).unwrap();
        g.best_path(start, end);
        assert_eq!(102, g[12][12].traversal.as_ref().unwrap().total_heat_loss);
    }
}
