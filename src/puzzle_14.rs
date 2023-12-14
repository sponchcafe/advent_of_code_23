use crate::util::load_lines;

pub fn puzzle_14_1() -> u64 {
    let lines: Vec<String> = load_lines("14/input.txt")
        .collect::<Result<Vec<String>, _>>()
        .expect("valid input");

    transpose(&lines)
        .iter()
        .map(|s| weight_after_shift(s))
        .sum::<usize>() as u64
}

fn transpose(lines: &Vec<String>) -> Vec<String> {
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

fn weight_after_shift(s: &str) -> usize {
    let south_dist = s.len();
    s.chars()
        .enumerate()
        .scan(south_dist, |weight, (i, rock)| match rock {
            'O' => {
                let w = *weight;
                *weight -= 1;
                Some(w)
            }
            '#' => {
                *weight = south_dist - (i + 1);
                Some(0)
            }
            _ => Some(0),
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transpose() {
        let input: Vec<String> = ["123", "456", "789"]
            .into_iter()
            .map(String::from)
            .collect();

        let expected: Vec<String> = ["147", "258", "369"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(expected, transpose(&input));
        assert_eq!(input, transpose(&expected));
        assert_eq!(input, transpose(&transpose(&input)));
    }

    #[test]
    fn test_weights() {
        let input = "..O#O.O..O.##O.O..#.#.OOOO...O#O";
        // Shifted: "O..#OOO....##OO...#.#OOOOO....#O";
        // Weights: "32  28       19      11 87     1
        //                27       18      10
        //                 26               9
        // Sum = 32+28+27+26+19+18+11+10+9+8+7+1 = 196

        assert_eq!(196, weight_after_shift(&input));
    }
}
