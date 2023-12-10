use crate::util::load_file;
use anyhow::Error;
use std::ops::Index;
use std::str::FromStr;

pub fn puzzle_10_1() -> usize {
    load_file("10/input.txt")
        .parse::<Pipes>()
        .expect("valid pipe input")
        .pathlength()
}

pub fn puzzle_10_2() -> u64 {
    0
}

#[derive(Debug, PartialEq)]
enum Pipe {
    Start,      // S
    WestEast,   // -
    NorthSouth, // |
    NorthWest,  // J
    NorthEast,  // L
    SouthWest,  // 7
    SouthEast,  // F
    None,       // .
}

impl Pipe {
    fn new(c: char) -> Self {
        use Pipe::*;
        match c {
            'S' => Start,
            '-' => WestEast,
            '|' => NorthSouth,
            'J' => NorthWest,
            'L' => NorthEast,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Pipes {
    rowlen: usize,
    data: Vec<Pipe>,
}

type Coordinate = (usize, usize);
type Connections = [Coordinate; 2];
type Step = [Coordinate; 2];

impl Index<Coordinate> for Pipes {
    type Output = Pipe;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.data[index.0 * self.rowlen + index.1]
    }
}

impl FromStr for Pipes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rowlen = s.find("\n").unwrap_or(0);
        let data = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Pipe::new)
            .collect();
        Ok(Pipes { rowlen, data })
    }
}

impl Pipes {
    fn connected(&self, coord: Coordinate) -> Connections {
        use Pipe::*;
        match self[coord] {
            Start => self.start_connections(coord), // S
            WestEast => [(coord.0, coord.1 - 1), (coord.0, coord.1 + 1)], // -
            NorthSouth => [(coord.0 - 1, coord.1), (coord.0 + 1, coord.1)], // \
            NorthWest => [(coord.0 - 1, coord.1), (coord.0, coord.1 - 1)], // J
            NorthEast => [(coord.0 - 1, coord.1), (coord.0, coord.1 + 1)], // L
            SouthWest => [(coord.0, coord.1 - 1), (coord.0 + 1, coord.1)], // 7
            SouthEast => [(coord.0, coord.1 + 1), (coord.0 + 1, coord.1)], // F
            None => [coord, coord],
        }
    }

    fn start_connections(&self, coord: Coordinate) -> Connections {
        let mut connected = vec![];
        for r in coord.0 - 1..=coord.0 + 1 {
            for c in coord.1 - 1..=coord.1 + 1 {
                if r == coord.0 && c == coord.1 {
                    continue;
                }
                let incoming = self.connected((r, c));
                if incoming[0] == coord || incoming[1] == coord {
                    connected.push((r, c));
                }
            }
        }
        [connected[0], connected[1]]
    }

    fn start(&self) -> Coordinate {
        let pos = self
            .data
            .iter()
            .position(|p| p == &Pipe::Start)
            .expect("there should be a start");
        (pos % self.rowlen, pos / self.rowlen)
    }

    fn step(&self, step: Step) -> Step {
        let next_connections = self.connected(step[1]);
        if next_connections[0] == step[0] {
            [step[1], next_connections[1]]
        } else {
            [step[1], next_connections[0]]
        }
    }

    fn pathlength(&self) -> usize {
        let start: Coordinate = self.start();
        let start_connections = self.connected(start);
        let mut step = [start, start_connections[0]];
        let mut len = 0;
        loop {
            step = self.step(step);
            len += 1;
            if step[0] == start {
                return len / 2;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("data/10/example.txt");

    #[test]
    fn test_parse_pipes() {
        let pipes = EXAMPLE.parse::<Pipes>().unwrap();
        assert_eq!(Pipe::None, pipes[(0, 0)]);
        assert_eq!(Pipe::None, pipes[(2, 2)]);
        assert_eq!(Pipe::None, pipes[(4, 4)]);

        assert_eq!(Pipe::Start, pipes[(1, 1)]);
        assert_eq!(Pipe::WestEast, pipes[(1, 2)]);
        assert_eq!(Pipe::SouthWest, pipes[(1, 3)]);
        assert_eq!(Pipe::NorthSouth, pipes[(2, 3)]);
        assert_eq!(Pipe::NorthWest, pipes[(3, 3)]);
        assert_eq!(Pipe::WestEast, pipes[(3, 2)]);
        assert_eq!(Pipe::NorthEast, pipes[(3, 1)]);
        assert_eq!(Pipe::NorthSouth, pipes[(2, 1)]);
    }

    #[test]
    fn test_connections() {
        let pipes = EXAMPLE.parse::<Pipes>().unwrap();
        assert_eq!([(0, 0), (0, 0)], pipes.connected((0, 0)));

        assert_eq!([(1, 2), (2, 1)], pipes.connected((1, 1)));
        assert_eq!([(1, 1), (1, 3)], pipes.connected((1, 2)));
        assert_eq!([(1, 2), (2, 3)], pipes.connected((1, 3)));

        assert_eq!([(1, 1), (3, 1)], pipes.connected((2, 1)));
        assert_eq!([(1, 3), (3, 3)], pipes.connected((2, 3)));

        assert_eq!([(2, 1), (3, 2)], pipes.connected((3, 1)));
        assert_eq!([(3, 1), (3, 3)], pipes.connected((3, 2)));
        assert_eq!([(2, 3), (3, 2)], pipes.connected((3, 3)));
    }

    #[test]
    fn test_get_start() {
        let pipes = EXAMPLE.parse::<Pipes>().unwrap();
        assert_eq!((1, 1), pipes.start());
    }

    #[test]
    fn test_step() {
        let pipes = EXAMPLE.parse::<Pipes>().unwrap();
        let start = pipes.start();
        let start_connections = pipes.connected(start);
        let step = [start, start_connections[0]];
        let next = pipes.step(step);
        let expected = [(1, 2), (1, 3)];
        assert_eq!(expected, next);

        let next = pipes.step(next);
        let expected = [(1, 3), (2, 3)];
        assert_eq!(expected, next)
    }

    #[test]
    fn test_example() {
        let pipes = EXAMPLE.parse::<Pipes>().unwrap();
        assert_eq!(4, pipes.pathlength());
    }
}
