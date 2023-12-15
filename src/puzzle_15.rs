use crate::util::load_file;
use anyhow::Error;
use std::num::Wrapping;
use std::str::FromStr;

pub fn puzzle_15_1() -> u64 {
    let input = load_file("15/input.txt");
    let ops: Vec<&str> = input.split(",").map(str::trim).collect();
    ops.iter().map(|s| hash(&s) as u64).sum()
}

pub fn puzzle_15_2() -> u64 {
    let input = load_file("15/input.txt");
    let ops: Vec<Operation> = input
        .split(",")
        .map(|s| s.trim().parse::<Operation>())
        .collect::<Result<Vec<Operation>, _>>()
        .expect("valid operations");

    let mut hashmap: [Vec<Lens>; 0x100] = std::array::from_fn(|_| Vec::new());

    for op in ops {
        let hashbox = &mut hashmap[op.hash as usize];
        let existing_lens: Option<(usize, &mut Lens)> = hashbox
            .iter_mut()
            .enumerate()
            .filter(|(_, l)| l.label == op.label)
            .next();
        match (op.op, existing_lens) {
            // Ovewrite
            (Op::Insert(focal_length), Some((_, l))) => {
                *l = Lens {
                    label: op.label,
                    focal_length,
                };
            }
            // Insert
            (Op::Insert(focal_length), None) => {
                hashbox.push(Lens {
                    label: op.label,
                    focal_length,
                });
            }
            // Remove
            (Op::Remove, Some((i, _))) => {
                hashbox.remove(i);
            }
            // Remove nonexistent => noop
            _ => {}
        }
    }

    let mut total_sum = 0usize;
    for (box_idx, hashbox) in hashmap.iter().enumerate() {
        total_sum += hashbox.iter().enumerate().fold(0, |sum, (lens_idx, lens)| {
            sum + (box_idx + 1) * (lens_idx + 1) * lens.focal_length as usize
        });
    }

    total_sum as u64
}

fn hash(s: &str) -> u8 {
    s.bytes()
        .into_iter()
        .fold(Wrapping::<u8>(0), |h, c| (h + Wrapping(c)) * Wrapping(17))
        .0
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Op {
    Insert(u32),
    Remove,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Operation {
    hash: u8,
    label: String,
    op: Op,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((label, right)) = s.split_once('=') {
            Ok(Operation {
                hash: hash(label),
                label: label.to_string(),
                op: Op::Insert(right.parse::<u32>()?),
            })
        } else {
            let label = s.trim_end_matches('-');
            Ok(Operation {
                hash: hash(label),
                label: label.to_string(),
                op: Op::Remove,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(30, hash("rn=1"));
        assert_eq!(253, hash("cm-"));
        assert_eq!(97, hash("qp=3"));
        assert_eq!(0, hash(""));
    }

    #[test]
    fn parse_operation() {
        assert_eq!(
            Operation {
                hash: hash("rn"),
                label: "rn".to_string(),
                op: Op::Insert(1),
            },
            "rn=1".parse::<Operation>().unwrap()
        );

        assert_eq!(
            Operation {
                hash: hash("cm"),
                label: "cm".to_string(),
                op: Op::Remove,
            },
            "cm-".parse::<Operation>().unwrap()
        );
    }
}
