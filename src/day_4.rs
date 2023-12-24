use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT: &str = "inputs/input_4.txt";

pub fn main() {
    let file = File::open(INPUT).unwrap();
    let mut cards = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let card = Card::from_line(&line);
        if let Some(card) = card {
            cards.push(card);
        } else {
            println!("Cannot parse line {line}");
        }
    }
    let point_ttl: usize = cards.iter().map(|c| c.score()).sum();
    println!("Part 1: Total points: {point_ttl}");

    win_copies(&mut cards[..]);
    let ttl_cards: usize = cards.iter().map(|c| c.copies).sum();
    println!("Part 2: Total scratchcards: {ttl_cards}");
}

#[allow(dead_code)]
struct Card {
    num: usize,
    winning_nums: HashSet<usize>,
    have_nums: Vec<usize>,
    copies: usize,
}

impl Card {
    fn from_line(line: &str) -> Option<Card> {
        lazy_static! {
            static ref OK_LINE_RE: Regex = Regex::new(r"Card +(\d+):([\d ]+)\|([\d ]+)").unwrap();
        }

        let caps = OK_LINE_RE.captures(line);
        if let Some(caps) = caps {
            let num: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let winning_nums: HashSet<usize> = caps
                .get(2)
                .unwrap()
                .as_str()
                .trim()
                .split(' ')
                .filter_map(|s| {
                    if s.len() > 0 {
                        Some(s.parse().unwrap())
                    } else {
                        None
                    }
                })
                .collect();
            let have_nums: Vec<usize> = caps
                .get(3)
                .unwrap()
                .as_str()
                .trim()
                .split(' ')
                .filter_map(|s| {
                    if s.len() > 0 {
                        Some(s.parse().unwrap())
                    } else {
                        None
                    }
                })
                .collect();
            Some(Card {
                num,
                winning_nums,
                have_nums,
                copies: 1,
            })
        } else {
            None
        }
    }

    fn num_matches(&self) -> i32 {
        let mut matches: i32 = 0;
        for num in self.have_nums.iter() {
            if self.winning_nums.contains(&num) {
                matches += 1;
                // println!("{num} is a winning number!");
            }
        }
        matches
    }

    fn score(&self) -> usize {
        let matches = self.num_matches();
        if matches > 0 {
            2usize.pow((matches - 1) as u32)
        } else {
            0
        }
    }
}

fn win_copies(cards: &mut [Card]) {
    for idx in 0..cards.len() {
        let matches = cards[idx].num_matches() as usize;
        for idx2 in idx + 1..idx + matches + 1 {
            if idx2 >= cards.len() {
                break;
            }
            cards[idx2].copies += cards[idx].copies;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;
    const EXAMPLE_1: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_score() {
        let results = vec![8, 2, 2, 1, 0, 0];
        for (line, expected_score) in zip(EXAMPLE_1.lines(), results.iter()) {
            let card = Card::from_line(line).unwrap();
            assert_eq!(card.score(), *expected_score);
        }
    }

    #[test]
    fn test_copies() {
        let results = vec![1, 2, 4, 8, 14, 1];
        let mut cards: Vec<Card> = EXAMPLE_1
            .lines()
            .filter_map(|l| Card::from_line(l))
            .collect();
        win_copies(&mut cards[..]);
        for (card, expected_count) in zip(cards, results) {
            assert_eq!(card.copies, expected_count);
        }
    }
}
