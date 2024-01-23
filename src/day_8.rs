use crate::utils;
use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
// use std::num::Integer;

const INPUT: &str = "inputs/input_8.txt";

pub fn main() {
    let docs = Docs::from_file(utils::lines_in_file(INPUT)).unwrap();
    let steps = docs.steps_to_zzz();
    println!("Reaching ZZZ in {steps} steps");

    let aa_steps = docs.steps_from_all_a_to_any_z();
    println!("Going from ??A to ??Z in {aa_steps} steps");
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
            static ref MAP_RE: Regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
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

    fn steps_from_all_a_to_any_z(&self) -> u64 {
        let mut steps = 0;
        let mut locs: Vec<&String> = self.map.keys().filter(|loc| loc.ends_with("A")).collect();
        let mut paths: Vec<Path> = locs
            .iter()
            .map(|l| Path::new(l, locs.len() as u64))
            .collect();
        let mut instructions = Box::new(self.instructions.iter());
        println!("Locs: {locs:?}");
        loop {
            let direction = if let Some(i) = instructions.next() {
                i
            } else {
                *instructions = self.instructions.iter(); // go back through
                instructions.next().unwrap() // there had better be a first direction to go
            };
            steps += 1;
            for (loc, path) in locs.iter_mut().zip(paths.iter_mut()) {
                if !path.found_loop() {
                    let lr = self.map.get(*loc).unwrap();
                    *loc = &lr[*direction as usize];
                    if loc.ends_with("Z") {
                        path.at_xxz(loc, steps);
                    }
                }
            }
            if paths.iter().all(|p| p.found_loop()) {
                break;
            }
        }
        for path in paths.iter() {
            path.print_found();
        }
        let strides: Vec<u64> = paths
            .iter()
            .filter_map(|p| p.simplest_repetitions())
            .collect();
        if strides.len() == paths.len() {
            return get_lcm(&strides);
        }
        panic!("Cannot handle anything but the very simplest repetive sequences.");
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

fn get_lcm(strides: &[u64]) -> u64 {
    strides.iter().fold(1, |acc, i| acc.lcm(i))
}

struct Path {
    start: String,
    xxzs: Vec<Terminal>, // steps from start when an ??Z node was visited
    instr_len: u64,
    loop_offset: u64, // 0 until a loop has been found
    loop_length: u64, // ditto
}

struct Terminal {
    step: u64,
    instr: u64,
    name: String,
}

impl Path {
    fn new(start: &str, instr_len: u64) -> Path {
        Path {
            start: start.to_owned(),
            xxzs: Vec::new(),
            instr_len,
            loop_length: 0,
            loop_offset: 0,
        }
    }

    // Returns true if a loop has been detected.
    fn at_xxz(&mut self, loc: &str, step: u64) {
        if self.found_loop() {
            return;
        }
        let instr = step % self.instr_len;
        println!(
            "Path starting from {} reached {} on step {}, instruction {}",
            self.start, loc, step, instr
        );

        // Check for a loop.
        for xxz in self.xxzs.iter() {
            if xxz.name == loc && xxz.instr == instr {
                // Same place in instruction sequence as before
                self.loop_length = step - xxz.step;
                self.loop_offset = xxz.step;
                println!("   We're looping!");
                break; // Small optimization, makes me happier too.
            }
        }

        self.xxzs.push(Terminal {
            step,
            instr,
            name: loc.to_owned(),
        });
    }

    fn found_loop(&self) -> bool {
        self.loop_length != 0
    }

    fn print_found(&self) {
        print!("Starting at {}:", self.start);
        for xxz in self.xxzs.iter() {
            print!(" {} step {} instr {},", xxz.name, xxz.step, xxz.instr);
        }
        println!();
    }

    fn simplest_repetitions(&self) -> Option<u64> {
        let (first, remaining) = self.xxzs.split_first().unwrap();
        if !remaining.iter().all(|xxz| first.name == xxz.name) {
            return None;
        }
        let stride = self.xxzs[0].step;
        for (idx, xxz) in self.xxzs.iter().enumerate().skip(1) {
            if xxz.step != (idx as u64 + 1) * stride {
                return None;
            }
        }
        Some(stride)
    }
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

    const EXAMPLE_3: &str = r#"LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)"#;

    #[test]
    fn test_part1_examples() {
        let docs = Docs::from_file(EXAMPLE_1.lines()).unwrap();
        assert_eq!(docs.map.len(), 7);
        assert_eq!(docs.steps_to_zzz(), 2);

        let docs2 = Docs::from_file(EXAMPLE_2.lines()).unwrap();
        assert_eq!(docs2.steps_to_zzz(), 6);
    }

    #[test]
    fn test_pt2_example() {
        let docs = Docs::from_file(EXAMPLE_3.lines()).unwrap();
        assert_eq!(docs.steps_from_all_a_to_any_z(), 6);
    }
}
