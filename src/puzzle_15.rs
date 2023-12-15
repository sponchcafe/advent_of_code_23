use crate::util::load_file;
use std::num::Wrapping;

pub fn puzzle_15_1() -> u64 {
    let input = load_file("15/input.txt");
    let items: Vec<&str> = input.split(",").map(str::trim).collect();
    items.iter().map(|s| init_hash(&s) as u64).sum()
}

fn init_hash(s: &str) -> u8 {
    s.bytes()
        .into_iter()
        .fold(Wrapping::<u8>(0), |h, c| (h + Wrapping(c)) * Wrapping(17))
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(30, init_hash("rn=1"));
        assert_eq!(253, init_hash("cm-"));
        assert_eq!(97, init_hash("qp=3"));
        assert_eq!(0, init_hash(""));
    }
}
