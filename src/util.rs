use std::fs::File;
use std::io;
use std::io::BufRead;
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
