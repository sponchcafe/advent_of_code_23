use crate::util::load_lines;

pub fn puzzle_9_1() -> i32 {
    load_lines("9/input.txt")
        .map(|l| parse_line(&l.expect("readline")))
        .map(|n| prediction(&n[..]))
        .sum::<i32>()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split(char::is_whitespace)
        .map(|s| str::parse::<i32>(s).expect("valid numbers"))
        .collect()
}

fn prediction(data: &[i32]) -> i32 {
    let mut data: Vec<i32> = data.iter().cloned().collect();
    let mut end: usize = data.len() - 1;

    loop {
        if data[..end].iter().all(|v| *v == 0) {
            return data.iter().sum();
        }
        for i in 0..end {
            data[i] = data[i + 1] - data[i];
        }
        end -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(0, prediction(&[0, 0, 0]));
    }

    const EXAMPLE_1: &[i32] = &[0, 3, 6, 9, 12, 15];
    const EXAMPLE_2: &[i32] = &[1, 3, 6, 10, 15, 21];
    const EXAMPLE_3: &[i32] = &[10, 13, 16, 21, 30, 45];

    #[test]
    fn test_examples() {
        assert_eq!(18, prediction(EXAMPLE_1));
        assert_eq!(28, prediction(EXAMPLE_2));
        assert_eq!(68, prediction(EXAMPLE_3));
    }
}
