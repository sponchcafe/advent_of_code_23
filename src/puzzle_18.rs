use crate::util::load_file;

pub fn puzzle_18_1() -> u64 {
    let input = load_file("18/input.txt");
    let path = Path::parse(&input);
    path.area() as u64
}

pub fn puzzle_18_2() -> u64 {
    let input = load_file("18/input.txt");
    let path = Path::parse_hex(&input);
    path.area() as u64
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    path: Vec<(isize, isize)>,
    circumference: usize,
}

impl Path {
    fn parse_hex(s: &str) -> Self {
        let mut circumference = 0usize;
        let path = s
            .trim()
            .lines()
            .map(|l| {
                let hex = l.trim().split(" ").skip(2).next().expect("hex part");
                (
                    isize::from_str_radix(&hex[2..7], 16).expect("distance"),
                    &hex[7..8],
                )
            })
            .scan((0, 0), |state, (dist, dir)| {
                circumference += dist as usize;
                match dir {
                    "0" => state.0 += dist,
                    "1" => state.1 -= dist,
                    "2" => state.0 -= dist,
                    "3" => state.1 += dist,
                    _ => panic!("unexpected direction"),
                };
                Some(*state)
            })
            .collect();

        let mut path = Self::normalize_path(path);
        if path[0].1 != path[1].1 {
            path.rotate_left(1);
        }
        Path {
            path,
            circumference,
        }
    }

    fn parse(s: &str) -> Self {
        let mut circumference = 0usize;
        let path = s
            .trim()
            .lines()
            .map(|l| {
                let mut it = l.trim().split(" ");
                (it.next().expect("direction"), it.next().expect("distance"))
            })
            .scan((0, 0), |state, (dir, dist)| {
                let dist = dist.parse::<isize>().expect("invalid distance");
                circumference += isize::abs(dist) as usize;
                match dir {
                    "R" => state.0 += dist,
                    "D" => state.1 -= dist,
                    "L" => state.0 -= dist,
                    "U" => state.1 += dist,
                    _ => panic!("unexpected direction"),
                };
                Some(*state)
            })
            .collect();
        let mut path = Self::normalize_path(path);
        if path[0].1 != path[1].1 {
            path.rotate_left(1);
        }
        Path {
            path,
            circumference,
        }
    }

    fn normalize_path(path: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
        let minx = path.iter().map(|(x, _)| *x).min().expect("minx");
        let miny = path.iter().map(|(_, y)| *y).min().expect("miny");
        path.into_iter()
            .map(|(x, y)| ((x - minx), (y - miny)))
            .collect()
    }

    fn area(&self) -> usize {
        let inner = self
            .path
            .chunks(2)
            .fold(0, |sum, chunk| sum + (chunk[1].0 - chunk[0].0) * chunk[0].1);
        (inner + self.circumference as isize / 2 + 1) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)";

    #[test]
    fn test_parse_normalized() {
        let expected = Path {
            path: vec![
                (6, 4),
                (4, 4),
                (4, 2),
                (6, 2),
                (6, 0),
                (1, 0),
                (1, 2),
                (0, 2),
                (0, 4),
                (2, 4),
                (2, 7),
                (0, 7),
                (0, 9),
                (6, 9),
            ],
            circumference: 38,
        };

        assert_eq!(expected, Path::parse(EXAMPLE));
    }

    #[test]
    fn test_area() {
        assert_eq!(62, Path::parse(EXAMPLE).area());
    }
}
