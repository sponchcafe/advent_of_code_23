use crate::util::load_lines;
use ::std::collections::BTreeSet;
use anyhow::Error as AnyhowError;
use std::cell::Cell;
use std::str::FromStr;

pub fn puzzle_4_1() -> u32 {
    load_lines("4/input.txt")
        .map(|l| Card::from_str(&l.expect("read line")).expect("valid card"))
        .map(|c| c.score())
        .sum()
}

pub fn puzzle_4_2() -> u32 {
    let mut wins: Vec<u32> = load_lines("4/input.txt")
        .map(|l| {
            Card::from_str(&l.expect("read line"))
                .expect("valid card")
                .hits()
        })
        .collect();
    multiply_cards(&mut wins);
    wins.iter().sum()
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning: BTreeSet<u32>,
    scored: BTreeSet<u32>,
}

#[derive(Debug, PartialEq)]
struct MultiCard {
    count: Cell<u64>,
    card: Card,
}

impl Card {
    fn score(&self) -> u32 {
        let hits = self.hits();
        match hits {
            0..=1 => hits as u32,
            n => 2u32.pow((n - 1) as u32),
        }
    }
    fn hits(&self) -> u32 {
        self.scored.intersection(&self.winning).count() as u32
    }
}

fn multiply_cards(winning: &mut [u32]) {
    let mut count = vec![1u32; winning.len()];
    for i in 0..winning.len() {
        for j in (i + 1)..=(i + (winning[i] as usize)) {
            if j >= winning.len() {
                break;
            }
            count[j] += count[i];
        }
    }
    winning.clone_from_slice(&count[..]);
}

impl FromStr for Card {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(":");
        let card_name = it.next().ok_or(Self::Err::msg("no name"))?.trim();
        let numbers = it.next().ok_or(Self::Err::msg("no numbers"))?.trim();
        let id = card_name
            .split_whitespace()
            .last()
            .ok_or(Self::Err::msg("no id"))?
            .parse::<u32>()
            .or(Err(Self::Err::msg("invalid id")))?;
        let mut it = numbers.split("|");
        let winning = it
            .next()
            .ok_or(Self::Err::msg("no separator"))?
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<BTreeSet<u32>, _>>()
            .or(Err(Self::Err::msg("invalid winning numbers")))?;
        let scored = it
            .last()
            .ok_or(Self::Err::msg("no seperator"))?
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<BTreeSet<u32>, _>>()
            .or(Err(Self::Err::msg("invalid scored numbers")))?;

        Ok(Card {
            id,
            winning,
            scored,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 12  1 23 34 | 55 11 0";
        let winning: BTreeSet<u32> = [1, 12, 23, 34].into_iter().collect();
        let scored: BTreeSet<u32> = [55, 11, 0].into_iter().collect();
        let expected = Card {
            id: 1,
            winning,
            scored,
        };
        let parsed = Card::from_str(input).unwrap();
        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_score() {
        let winning: BTreeSet<u32> = (0..10).into_iter().collect();
        let scored: BTreeSet<u32> = [11, 23, 55].into_iter().collect();
        let card = Card {
            id: 0,
            winning,
            scored,
        };
        assert_eq!(0, card.score());

        let winning: BTreeSet<u32> = [1, 12, 23, 34].into_iter().collect();
        let scored: BTreeSet<u32> = [1, 11, 23, 55].into_iter().collect();
        let card = Card {
            id: 1,
            winning,
            scored,
        };
        assert_eq!(2, card.score());

        let winning: BTreeSet<u32> = (0..10).into_iter().collect();
        let scored: BTreeSet<u32> = (1..5).into_iter().collect();
        let card = Card {
            id: 2,
            winning,
            scored,
        };
        assert_eq!(8, card.score());
    }

    #[test]
    fn test_multiply_cards() {
        let mut input = [4, 2, 2, 1, 0, 0];
        //          [1, 1, 1, 1, 1, 1]
        //          [1, 2, 2, 2, 2, 1]
        //          [1, 2, 4, 4, 2, 1]
        //          [1, 2, 4, 8, 6, 1]
        //          [1, 2, 4, 8, 14, 1]
        let expected = [1, 2, 4, 8, 14, 1];
        multiply_cards(&mut input[..]);

        assert_eq!(expected, input);
    }
}
