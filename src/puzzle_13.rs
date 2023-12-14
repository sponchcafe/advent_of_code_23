use crate::util::{load_lines, transpose};

pub fn puzzle_13_1() -> u64 {
    let lines: Vec<String> = load_lines("13/input.txt")
        .collect::<Result<Vec<String>, _>>()
        .expect("valid input");

    calc_score(lines)
}

fn calc_score(lines: Vec<String>) -> u64 {
    let mut total_score = 0;
    let mut pattern = vec![];
    for l in lines {
        if !l.is_empty() {
            pattern.push(l.clone());
        } else {
            dbg!(pattern.len());
            total_score += Pattern::new(&pattern).score();
            pattern.clear();
        }
    }
    total_score += Pattern::new(&pattern).score();

    total_score
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Pattern {
    fn new(data: &Vec<String>) -> Self {
        let parser = |s: &String| return str_to_num(&s[..], '#');
        let rows: Vec<u32> = data.iter().map(parser).collect();
        let cols: Vec<u32> = transpose(data).iter().map(parser).collect();
        Pattern { rows, cols }
    }

    fn score(&self) -> u64 {
        let cmp = |(&a, &b)| a == b;
        let c = find_palindrome(&self.cols, cmp).unwrap_or(0);
        let r = find_palindrome(&self.rows, cmp).unwrap_or(0);
        dbg!(&self.rows, &self.cols, r, c);
        (c + 100 * r) as u64
    }
}

fn find_palindrome<'a, T, P>(s: &'a [T], cmp: P) -> Option<usize>
where
    T: Eq + 'a,
    P: Fn((&'a T, &'a T)) -> bool,
{
    for i in 0..s.len() - 1 {
        if s[0..=i].iter().rev().zip(s[i + 1..].iter()).all(&cmp) {
            return Some(i + 1);
        }
    }
    None
}

fn str_to_num(s: &str, marker: char) -> u32 {
    s.chars()
        .rev()
        .enumerate()
        .fold(0, |num, (i, c)| num + if c == marker { 1 << i } else { 0 })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let lines: Vec<String> = load_lines("13/example.txt")
            .collect::<Result<Vec<String>, _>>()
            .expect("valid input");

        assert_eq!(405, calc_score(lines));
    }
}
