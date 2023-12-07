use crate::util::load_lines;
use anyhow::Error;
use std::collections::BTreeMap;

pub fn puzzle_7_1() -> u64 {
    parse_hands(false).total_winnings()
}

pub fn puzzle_7_2() -> u64 {
    parse_hands(true).total_winnings()
}

fn parse_hands(jokers: bool) -> Hands {
    let hands = load_lines("7/input.txt")
        .map(Result::unwrap)
        .map(|s| Hand::parse(&s, jokers))
        .collect::<Result<Vec<Hand>, _>>()
        .expect("valid hands");
    Hands::new(hands)
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct HandCards {
    cards: [Card; 5],
}

impl HandCards {
    fn new(s: &str, jokers: bool) -> Self {
        let cards: Vec<Card> = s.chars().map(|c| Card::new(c, jokers)).collect();
        let mut card_arr = [Card::default(); 5];
        card_arr[..].copy_from_slice(&cards[..]);
        HandCards { cards: card_arr }
    }

    fn counts_to_kind(counts: Counts) -> HandKind {
        use HandKind::*;
        match counts {
            [5, ..] => FiveOfAKind,
            [4, ..] => FourOfAKind,
            [3, 2, ..] => FullHouse,
            [3, ..] => ThreeOfAKind,
            [2, 2, ..] => TwoPair,
            [2, ..] => Pair,
            _ => HighCard,
        }
    }

    fn kind(&self) -> HandKind {
        let mut counter = Counter::new(&self.cards);
        counter.apply_joker_rule(&Card::Joker);
        Self::counts_to_kind(counter.counts())
    }
}

type Counts = [usize; 5];

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Card {
    Joker,
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum HandKind {
    #[default]
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn new(c: char, jokers: bool) -> Card {
        use Card::*;
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => {
                if jokers {
                    Joker
                } else {
                    Jack
                }
            }
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => panic!("no such card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: HandCards,
    kind: HandKind,
    bid: u32,
}

struct Hands(Vec<Hand>);

impl Hands {
    fn new(mut hands: Vec<Hand>) -> Self {
        hands.sort();
        Hands(hands)
    }

    fn total_winnings(&self) -> u64 {
        self.0.iter().enumerate().fold(0, |total, (i, hand)| {
            let rank = i as u64 + 1;
            total + rank * hand.bid as u64
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.kind, self.cards).partial_cmp(&(other.kind, other.cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    fn new(cards: &str, bid: u32, jokers: bool) -> Hand {
        let cards = HandCards::new(cards, jokers);
        Hand {
            cards,
            kind: cards.kind(),
            bid,
        }
    }
}

impl Hand {
    fn parse(s: &str, jokers: bool) -> Result<Self, Error> {
        let mut it = s.split(" ");
        let cards = it.next().ok_or(Error::msg("no cards"))?.trim();
        let bid = it.next().ok_or(Error::msg("no bid"))?.parse::<u32>()?;
        Ok(Hand::new(cards, bid, jokers))
    }
}

struct Counter<T>
where
    T: Clone + Ord,
{
    counts: BTreeMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Clone + Ord,
{
    fn new(data: &[T]) -> Self {
        let mut counts = BTreeMap::<T, usize>::new();
        data.into_iter().for_each(|item| {
            if counts.contains_key(&item) {
                *(counts.get_mut(&item).unwrap()) += 1;
            } else {
                counts.insert(item.clone(), 1);
            }
        });
        Counter { counts }
    }

    fn counts(&self) -> Counts {
        let mut all_counts: Vec<usize> = self.counts.values().map(|c| *c).collect();
        let mut counts: Counts = [0usize; 5];
        all_counts.sort_by(|a, b| b.cmp(a));
        counts[..all_counts.len()].copy_from_slice(&all_counts[..]);
        counts
    }

    fn apply_joker_rule(&mut self, joker: &T) {
        let jokers = self.counts.remove(joker).unwrap_or(0);
        if self.counts.is_empty() {
            self.counts.insert(joker.clone(), jokers);
            return;
        }
        if let Some(max_count) = self.counts.values_mut().max() {
            *max_count += jokers;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_hand() {
        let cards = HandCards {
            cards: [Card::Six, Card::Ten, Card::King, Card::Four, Card::Queen],
        };
        let bid = 440;
        let expected = Hand {
            cards,
            kind: HandKind::HighCard,
            bid,
        };
        let actual = Hand::parse("6TK4Q 440", false).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sort_cards() {
        assert!(Card::Two < Card::Three);
        assert!(Card::Five < Card::Nine);
        assert!(Card::King < Card::Ace);
    }

    #[test]
    fn test_sort_hands() {
        let cards1 = [Card::Six, Card::Ten, Card::King, Card::Four, Card::Queen];
        let cards2 = [Card::Six, Card::Ten, Card::Ace, Card::Four, Card::Queen];
        assert!(cards1 < cards2);
    }

    #[test]
    fn test_counter() {
        let cards = [Card::Six, Card::Ten, Card::King, Card::King, Card::Queen];
        let counter = Counter::new(&cards[..]);
        assert_eq!(1, *counter.counts.get(&Card::Six).unwrap());
        assert_eq!(2, *counter.counts.get(&Card::King).unwrap());

        let expected: Counts = [2usize, 1, 1, 1, 0];
        assert_eq!(expected, counter.counts());
    }

    #[test]
    fn test_counts_to_score() {
        let hands = [
            ("23456", HandKind::HighCard),
            ("22456", HandKind::Pair),
            ("22355", HandKind::TwoPair),
            ("22256", HandKind::ThreeOfAKind),
            ("22255", HandKind::FullHouse),
            ("22226", HandKind::FourOfAKind),
            ("22222", HandKind::FiveOfAKind),
        ];

        for (cards, kind) in hands.into_iter() {
            let cards = HandCards::new(cards, false);
            assert_eq!(kind, cards.kind());
        }
    }

    #[test]
    fn test_order_hands() {
        let hands: Vec<Hand> = [
            "246KT 1", "24TK3 1", "24TKJ 1", "3335Q 1", "333AK 1", "44442 1", "55552 1", "KKKKK 1",
            "AAAAA 1",
        ]
        .into_iter()
        .map(|s| Hand::parse(s, false))
        .collect::<Result<Vec<Hand>, _>>()
        .unwrap();

        assert!(Card::Four < Card::Jack);

        assert!(hands.windows(2).all(|w| {
            let cmp = w[0] < w[1];
            dbg!(&w[0], &w[1], cmp);
            cmp
        }));
    }

    #[test]
    fn test_example() {
        let input = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
        let hands = Hands::new(
            input
                .into_iter()
                .map(|s| Hand::parse(s, false))
                .collect::<Result<Vec<Hand>, _>>()
                .unwrap(),
        );
        assert_eq!(6440, hands.total_winnings());

        let hands = Hands::new(
            input
                .into_iter()
                .map(|s| Hand::parse(s, true))
                .collect::<Result<Vec<Hand>, _>>()
                .unwrap(),
        );
        assert_eq!(5905, hands.total_winnings());
    }

    #[test]
    fn test_jokers_apply() {
        let hand = Hand::parse("JJJJJ 1", true).unwrap();
        assert_eq!(HandKind::FiveOfAKind, hand.kind);
        assert_eq!(HandKind::FiveOfAKind, hand.kind);

        let hand = Hand::parse("234AK 1", true).unwrap();
        assert_eq!(HandKind::HighCard, hand.kind);
        assert_eq!(HandKind::HighCard, hand.kind);
    }
}
