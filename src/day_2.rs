use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use std::iter::zip;

// pub fn main() {
// }

struct Game {
    id: i32,
    reveals: Vec<CubeSet>,
}

#[derive(Clone)]
struct CubeSet {
    num_red: i32,
    num_green: i32,
    num_blue: i32,
}

impl Game {
    fn from_str(s: &str) -> Game {
        let id_results: Vec<&str> = s.split(":").collect();
        assert_eq!(id_results.len(), 2);
        let mut game_id = id_results[0].trim().split(" ");
        assert_eq!(game_id.next().unwrap(), "Game");
        let id = i32::from_str_radix(game_id.next().unwrap(), 10).unwrap();
        let reveals: Vec<CubeSet> = id_results[1].split(";").map(|S| CubeSet::from_str(s)).collect();
        Game{id, reveals}
    }

    fn minimal_bag(&self) -> CubeSet {
        let mut minimal = CubeSet{num_red: 0, num_green: 0, num_blue: 0};
        for reveal in self.reveals.iter() {
            minimal.expand_to_hold(reveal);
        }
        minimal
    }
}

impl CubeSet {
    fn from_str(s: &str) -> CubeSet {
        let mut cs = CubeSet{num_red: 0, num_green: 0, num_blue:0};
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
        self.num_red >= other.num_red && self.num_green >= other.num_green && self.num_blue >= other.num_blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_game_from_str() {
        let results = vec![true, true, false, false, true];
        let elf_bag = CubeSet{num_red: 12, num_green: 13, num_blue: 14};
        for (line, expected_result) in zip(EXAMPLE_1.lines(), results.iter()) {
            let g = Game::from_str(line);
            let min_bag = g.minimal_bag();
            assert_eq!(elf_bag.contains(&min_bag), *expected_result);
        }
    }
}