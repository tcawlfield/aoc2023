// use lazy_static::lazy_static;
// use regex::Regex;
use std::borrow::Borrow;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let file = File::open("inputs/input_2.txt").unwrap();
    let elf_bag = CubeSet {
        num_red: 12,
        num_green: 13,
        num_blue: 14,
    };
    let lines = io::BufReader::new(file).lines();
    let lines = lines.map(|l| l.expect("Bad line!"));
    let id_sum = id_sum_possible(&elf_bag, lines);
    println!("id_sum: {id_sum}");

    let file = File::open("inputs/input_2.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let lines = lines.map(|l| l.expect("Bad line!"));
    let power_sum = sum_powers(lines);
    println!("power_sum: {power_sum}");
}

fn id_sum_possible<I>(elf_bag: &CubeSet, lines: I) -> i32
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let mut id_sum: i32 = 0;
    for line in lines {
        let g = Game::from_str(line.borrow());
        if let Some(g) = g {
            let min_bag = g.minimal_bag();
            if elf_bag.contains(&min_bag) {
                id_sum += g.id;
            }
        }
    }
    id_sum
}

fn sum_powers<I>(lines: I) -> i32
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    let mut power_sum: i32 = 0;
    for line in lines {
        let g = Game::from_str(line.borrow());
        if let Some(g) = g {
            let min_bag = g.minimal_bag();
            power_sum += min_bag.get_power();
        }
    }
    power_sum
}

struct Game {
    id: i32,
    reveals: Vec<CubeSet>,
}

#[derive(Clone, Debug)]
struct CubeSet {
    num_red: i32,
    num_green: i32,
    num_blue: i32,
}

impl Game {
    fn from_str(s: &str) -> Option<Game> {
        let id_results: Vec<&str> = s.split(":").collect();
        if id_results.len() != 2 {
            println!("Bad line: {s:?}");
            return None;
        }
        let mut game_id = id_results[0].trim().split(" ");
        assert_eq!(game_id.next().unwrap(), "Game");
        let id = i32::from_str_radix(game_id.next().unwrap(), 10).unwrap();
        let reveals: Vec<CubeSet> = id_results[1]
            .split(";")
            .map(|s| CubeSet::from_str(s))
            .collect();
        Some(Game { id, reveals })
    }

    fn minimal_bag(&self) -> CubeSet {
        let mut minimal = CubeSet {
            num_red: 0,
            num_green: 0,
            num_blue: 0,
        };
        for reveal in self.reveals.iter() {
            minimal.expand_to_hold(reveal);
        }
        minimal
    }
}

impl CubeSet {
    fn from_str(s: &str) -> CubeSet {
        let mut cs = CubeSet {
            num_red: 0,
            num_green: 0,
            num_blue: 0,
        };
        for cubes in s.split(",") {
            let count_color: Vec<&str> = cubes.trim().split(" ").collect();
            assert_eq!(count_color.len(), 2);
            let count = i32::from_str_radix(count_color[0], 10).unwrap();
            match count_color[1] {
                "red" => cs.num_red += count,
                "green" => cs.num_green += count,
                "blue" => cs.num_blue += count,
                _ => panic!("Unknown color {}", count_color[1]),
            }
        }
        cs
    }

    // self.expand_to_hold(other) -> self = union(self, other)
    fn expand_to_hold(&mut self, other: &CubeSet) {
        self.num_red = cmp::max(self.num_red, other.num_red);
        self.num_green = cmp::max(self.num_green, other.num_green);
        self.num_blue = cmp::max(self.num_blue, other.num_blue);
    }

    fn contains(&self, other: &CubeSet) -> bool {
        self.num_red >= other.num_red
            && self.num_green >= other.num_green
            && self.num_blue >= other.num_blue
    }

    fn get_power(&self) -> i32 {
        self.num_red * self.num_green * self.num_blue
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    const EXAMPLE_1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    #[test]
    fn test_game_from_str() {
        let results = vec![true, true, false, false, true];
        let elf_bag = CubeSet {
            num_red: 12,
            num_green: 13,
            num_blue: 14,
        };
        for (line, expected_result) in zip(EXAMPLE_1.lines(), results.iter()) {
            let g = Game::from_str(line);
            if let Some(g) = g {
                let min_bag = g.minimal_bag();
                assert_eq!(elf_bag.contains(&min_bag), *expected_result);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_id_sum() {
        let elf_bag = CubeSet {
            num_red: 12,
            num_green: 13,
            num_blue: 14,
        };
        let id_sum = id_sum_possible(&elf_bag, EXAMPLE_1.lines());
        println!("test ID sum = {}", id_sum);
        assert_eq!(id_sum, 8);
    }

    #[test]
    fn day_2() {
        let results = vec![48, 12, 1560, 630, 36];
        for (line, expected_result) in zip(EXAMPLE_1.lines(), results.iter()) {
            let g = Game::from_str(line);
            if let Some(g) = g {
                let min_bag = g.minimal_bag();
                assert_eq!(min_bag.get_power(), *expected_result);
            }
        }

        assert_eq!(sum_powers(EXAMPLE_1.lines()), 2286);
    }
}
