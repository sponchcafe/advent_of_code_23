use std::fmt::{Display, Write};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::{Index, IndexMut};
use std::path::{Path, PathBuf};

const BASE_PATH: &str = env!("CARGO_MANIFEST_DIR");
const DATA_DIR: &str = "src/data/";

fn make_abs_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::new();
    abs_path.extend([BASE_PATH, DATA_DIR, path].map(Path::new));
    abs_path
}

/// Load a file from the puzzle data directory
#[allow(unused)]
pub fn load_file(path: &str) -> String {
    let abs_path = make_abs_path(path);
    std::fs::read_to_string(&abs_path).expect(&format!("Error reading: {:?}", &abs_path))
}

pub fn load_lines(path: &str) -> io::Lines<io::BufReader<File>> {
    let abs_path = make_abs_path(path);
    read_lines(abs_path).expect("file from lines")
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn transpose(lines: &Vec<String>) -> Vec<String> {
    let mut ret: Vec<String> = (0..lines[0].len())
        .into_iter()
        .map(|_| String::new())
        .collect();
    let mut iters: Vec<_> = lines.iter().map(|l| l.chars()).collect();
    for i in 0..lines[0].len() {
        ret[i] = iters
            .iter_mut()
            .map(|it| it.next().expect("equal line length"))
            .collect();
    }
    ret
}

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    pub shape: (usize, usize),
}

impl<T> Grid<T> {
    pub fn inspect(&self, f: impl Fn(&T) -> String) -> String {
        let mut s = String::new();
        for row in 0..self.shape.0 {
            for col in 0..self.shape.1 {
                s.push_str(&f(&self[row][col]));
            }
            s.push('\n');
        }
        s
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.shape.1..(index + 1) * self.shape.1]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.data[index * self.shape.1..(index + 1) * self.shape.1]
    }
}

impl<T> Grid<T>
where
    T: From<char>,
{
    pub fn from_table_data(s: &str) -> Self {
        let cols = s.find('\n').expect("reactangular line-separated data");
        let data: Vec<T> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.into())
            .collect();
        let rows = data.len() / cols;
        Grid {
            data,
            shape: (rows, cols),
        }
    }
}
impl<T> Grid<T>
where
    T: Clone,
{
    pub fn transpose(&mut self) {
        let mut transposed = vec![];
        self.shape = (self.shape.1, self.shape.0);
        for row in 0..self.shape.0 {
            for col in 0..self.shape.1 {
                let item = &self.data[row + self.shape.0 * col];
                transposed.push(item.clone());
            }
        }
        self.data = transposed;
    }
}

impl<T> Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in 0..self.shape.0 {
            for col in 0..self.shape.1 {
                f.write_fmt(format_args!("{} ", self[line][col]))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "123\n456\n789\nxyz";

    // 123
    // 456
    // 789
    // xyz
    //
    // 147x
    // 258y
    // 369z

    #[test]
    fn test_grid_create() {
        let grid = Grid::<char>::from_table_data(&DATA);
        assert_eq!('1', grid[0][0]);
        assert_eq!('5', grid[1][1]);
        assert_eq!('9', grid[2][2]);
        assert_eq!('2', grid[0][1]);
    }

    #[test]
    fn test_grid_transpose() {
        let mut grid = Grid::<char>::from_table_data(&DATA);
        grid.transpose();
        assert_eq!('1', grid[0][0]);
        assert_eq!('5', grid[1][1]);
        assert_eq!('9', grid[2][2]);
        assert_eq!('4', grid[0][1]);
        assert_eq!('y', grid[1][3]);
        assert_eq!('z', grid[2][3]);
    }
}
