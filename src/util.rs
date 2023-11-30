use std::path::Path;

/// Load a file from the puzzle data directory
pub fn load_file(path: &str) -> String {
    const BASE_PATH: &str = env!("CARGO_MANIFEST_DIR");
    const DATA_DIR: &str = "src/data/";
    let mut abs_path = std::path::PathBuf::new();
    abs_path.extend([BASE_PATH, DATA_DIR, path].map(Path::new));
    std::fs::read_to_string(&abs_path).expect(&format!("Error reading: {:?}", &abs_path))
}
