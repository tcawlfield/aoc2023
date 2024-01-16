use crate::utils;
use std::borrow::Borrow;

const INPUT: &str = "inputs/input_7.txt";

pub fn main() {
    let mut hands = hands_from_file(utils::lines_in_file(INPUT)).unwrap();
    let ttl_score = ttl_score(&mut hands);
    println!("Part 1: total score is {ttl_score}");
}

type Hands = Vec<Hand>;
// struct Hands {
//     hands: Vec<Hand>,
// }

struct Hand {
    cards: [u8; 5],
    bid: usize,
    typ: HandType,
    strength_bits: u64,
}

fn hands_from_file<I>(lines: I) -> Option<Hands>
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    Some(
        lines
            .into_iter()
            .filter_map(|l| Hand::from_line(l.borrow()))
            .collect(),
    )
}

fn ttl_score(hands: &mut Hands) -> usize {
    hands.sort_by_key(|s| s.strength_bits);
    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}

impl Hand {
    fn from_line(line: &str) -> Option<Hand> {
        let mut hand_bid = line.split_whitespace();
        let (hand_str, bid) = (hand_bid.next()?, hand_bid.next()?);
        let bid: usize = bid.parse().ok()?;
        let mut cards = [0; 5];
        let mut cards_in_hand = hand_str.chars();
        for idx in 0..5 {
            cards[idx] = card_value(cards_in_hand.next()?);
        }
        let typ = score_hand_type(&cards);
        let mut hand = Hand {
            cards,
            bid,
            typ,
            strength_bits: 0,
        };
        hand.get_str_bits();
        Some(hand)
    }

    fn get_str_bits(&mut self) {
        self.strength_bits = ((self.typ as u64) << 20)
            | (self.cards[0] as u64) << 16
            | (self.cards[1] as u64) << 12
            | (self.cards[2] as u64) << 8
            | (self.cards[3] as u64) << 4
            | (self.cards[4] as u64);
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum HandType {
    FiveOak = 7,
    FourOak = 6,
    FullHouse = 5,
    ThreeOak = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn score_hand_type(cards: &[u8; 5]) -> HandType {
    let mut shand = cards.clone();
    shand.sort();
    let mut of_a_kind = (0, 0);
    let mut current_matches = 0;
    let mut save_current = |current_matches| {
        if current_matches > 0 {
            if of_a_kind.0 == 0 {
                of_a_kind.0 = current_matches + 1; // 1 means two-of-a-kind, etc.
            } else {
                of_a_kind.1 = current_matches + 1;
            }
        }
    };
    for idx in 1..5 {
        if shand[idx] == shand[idx - 1] {
            current_matches += 1;
        } else {
            save_current(current_matches);
            current_matches = 0;
        }
    }
    save_current(current_matches);
    match of_a_kind {
        (5, 0) => HandType::FiveOak,
        (4, 0) => HandType::FourOak,
        (3, 2) => HandType::FullHouse,
        (2, 3) => HandType::FullHouse,
        (3, 0) => HandType::ThreeOak,
        (2, 2) => HandType::TwoPair,
        (2, 0) => HandType::OnePair,
        (0, 0) => HandType::HighCard,
        _ => panic!("Unknown hand type {of_a_kind:?}"),
    }
}

fn card_value(c: char) -> u8 {
    if c.is_ascii_digit() {
        return c.to_digit(10).unwrap() as u8;
    }
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0, // or panic?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::iter::zip;

    const EXAMPLE: &str = r#"32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"#;

    #[test]
    fn test_parse_example() {
        let mut hands = hands_from_file(EXAMPLE.lines()).unwrap();
        assert_eq!(hands.len(), 5);
        assert_eq!(hands[0].typ, HandType::OnePair);
        assert_eq!(hands[1].typ, HandType::ThreeOak);
        assert_eq!(hands[2].typ, HandType::TwoPair);
        assert_eq!(hands[3].typ, HandType::TwoPair);
        assert_eq!(hands[4].typ, HandType::ThreeOak);
        assert_eq!(ttl_score(&mut hands), 6440);
    }

    #[test]
    fn test_other_hand_types() {
        assert_eq!(Hand::from_line("12121 1").unwrap().typ, HandType::FullHouse);
        assert_eq!(Hand::from_line("21212 1").unwrap().typ, HandType::FullHouse);
        assert_eq!(Hand::from_line("55555 1").unwrap().typ, HandType::FiveOak);
        assert_eq!(Hand::from_line("55255 1").unwrap().typ, HandType::FourOak);
        assert_eq!(Hand::from_line("12345 1").unwrap().typ, HandType::HighCard);
    }
}
