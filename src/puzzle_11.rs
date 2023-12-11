use crate::util::load_lines;
use std::collections::BTreeSet;

pub fn puzzle_11_1() -> u64 {
    puzzle(2)
}

pub fn puzzle_11_2() -> u64 {
    puzzle(1000000)
}

fn puzzle(expansion_rate: usize) -> u64 {
    let lines = load_lines("11/input.txt");
    let universe = Universe::new(lines.map(|r| r.expect("line")), expansion_rate);
    universe.distances().into_iter().sum::<usize>() as u64
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Distance {
    from: Coordinate,
    to: Coordinate,
}

impl Distance {
    fn length(&self, expansion: &Expansion) -> usize {
        let start_row = usize::min(self.from.row, self.to.row);
        let stop_row = usize::max(self.from.row, self.to.row);
        let start_col = usize::min(self.from.col, self.to.col);
        let stop_col = usize::max(self.from.col, self.to.col);
        let mut delta_row = stop_row - start_row;
        let mut delta_col = stop_col - start_col;

        for e in expansion.rows.iter() {
            if start_row < *e && *e < stop_row {
                delta_row += expansion.rate - 1;
            }
        }

        for e in expansion.cols.iter() {
            if start_col < *e && *e < stop_col {
                delta_col += expansion.rate - 1;
            }
        }

        delta_row + delta_col
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: BTreeSet<Coordinate>,
    expansion: Expansion,
}

#[derive(Debug, PartialEq, Eq)]
struct Expansion {
    rows: Vec<usize>,
    cols: Vec<usize>,
    rate: usize,
}

impl Universe {
    fn new<L, S>(lines: L, expansion_rate: usize) -> Self
    where
        L: Iterator<Item = S>,
        S: AsRef<str>,
    {
        let mut galaxies = BTreeSet::new();
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in lines.enumerate() {
            max_col = line.as_ref().len();
            for (col, symbol) in line.as_ref().chars().enumerate() {
                if symbol == '#' {
                    galaxies.insert(Coordinate { row, col });
                }
            }
            max_row = row;
        }

        let expansion = Expansion {
            rows: (0..max_row)
                .filter(|&row| {
                    (0..max_col)
                        .find_map(|col| galaxies.get(&Coordinate { row, col }))
                        .is_none()
                })
                .collect(),
            cols: (0..max_row)
                .filter(|&col| {
                    (0..max_col)
                        .find_map(|row| galaxies.get(&Coordinate { row, col }))
                        .is_none()
                })
                .collect(),
            rate: expansion_rate,
        };

        Universe {
            galaxies,
            expansion,
        }
    }

    fn distances(&self) -> Vec<usize> {
        let mut galaxies = self.galaxies.clone();
        let mut distances = vec![];
        loop {
            if let Some(g0) = galaxies.pop_first() {
                for g1 in galaxies.iter() {
                    let dist = Distance {
                        from: g0.clone(),
                        to: g1.clone(),
                    };
                    distances.push(dist.length(&self.expansion));
                }
            } else {
                break;
            }
        }
        distances
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("data/11/example.txt");

    #[test]
    fn test_parse_universe() {
        let lines = EXAMPLE.lines();
        let universe = Universe::new(lines, 2);
        assert!(universe.galaxies.contains(&Coordinate { row: 0, col: 3 }));
        assert!(universe.galaxies.contains(&Coordinate { row: 1, col: 7 }));
        assert!(universe.galaxies.contains(&Coordinate { row: 9, col: 0 }));
    }

    #[test]
    fn test_expansion() {
        let lines = EXAMPLE.lines();
        let universe = Universe::new(lines, 2);
        let expected = Expansion {
            rows: vec![3, 7],
            cols: vec![2, 5, 8],
            rate: 2,
        };
        assert_eq!(expected, universe.expansion);
    }

    #[test]
    fn test_example() {
        let lines = EXAMPLE.lines();
        let universe = Universe::new(lines, 2);
        let distances: Vec<usize> = universe.distances();
        assert_eq!(374usize, distances.into_iter().sum());
    }

    #[test]
    fn test_example_rate() {
        let lines = EXAMPLE.lines();
        let universe = Universe::new(lines, 10);
        let distances: Vec<usize> = universe.distances();
        assert_eq!(1030usize, distances.into_iter().sum());
    }
}
