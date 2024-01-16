use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;

const INPUT: &str = "inputs/input_8.txt";

pub fn main() {
    let docs = Docs::from_file(utils::lines_in_file(INPUT)).unwrap();
    let steps = docs.steps_to_zzz();
    println!("Reaching ZZZ in {steps} steps");
}

struct Docs {
    // Using 0:u8 for L, and 1:u8 for R
    instructions: Vec<u8>,
    map: HashMap<String, [String; 2]>,
}

impl Docs {
    fn from_file<I>(lines: I) -> Option<Docs>
    where
        I: IntoIterator,
        I::Item: Borrow<str>,
    {
        lazy_static! {
            static ref MAP_RE: Regex =
                Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
        }
        let mut lines = lines.into_iter();
        let instructions = parse_instructions(lines.next()?.borrow());
        lines.next()?;
        let mut map = HashMap::new();
        for line in lines {
            let m = MAP_RE.captures(line.borrow()).unwrap();
            let key = m.get(1).unwrap().as_str().to_string();
            let left = m.get(2).unwrap().as_str().to_string();
            let right = m.get(3).unwrap().as_str().to_string();
            map.insert(key, [left, right]);
        }
        Some(Docs { instructions, map })
    }

    fn steps_to_zzz(&self) -> i32 {
        let end = "ZZZ";
        let mut steps = 0;
        let mut loc = "AAA";
        let mut instructions = Box::new(self.instructions.iter());
        loop {
            let direction = if let Some(i) = instructions.next() {
                i
            } else {
                *instructions = self.instructions.iter(); // go back through
                instructions.next().unwrap() // there had better be a first direction to go
            };
            let paths = self.map.get(loc).unwrap();
            loc = &paths[*direction as usize][..];
            steps += 1;
            if loc == end {
                break;
            }
        }
        steps
    }
}

fn parse_instructions(line: &str) -> Vec<u8> {
    line.chars()
        .filter_map(|c| match c {
            'L' => Some(0),
            'R' => Some(1),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::iter::zip;

    const EXAMPLE_1: &str = r#"RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)"#;

    const EXAMPLE_2: &str = r#"LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn test_parse_example() {
        let docs = Docs::from_file(EXAMPLE_1.lines()).unwrap();
        assert_eq!(docs.map.len(), 7);
        assert_eq!(docs.steps_to_zzz(), 2);

        let docs2 = Docs::from_file(EXAMPLE_2.lines()).unwrap();
        assert_eq!(docs2.steps_to_zzz(), 6);
    }
}
