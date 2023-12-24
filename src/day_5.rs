use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use crate::utils;

const INPUT: &str = "inputs/input_5.txt";

pub fn main() {
    let almanac = Almanac::from_file(utils::lines_in_file(INPUT)).unwrap();
    let closest_seed = almanac.min_loc();
    println!("Day 5 pt 1: Seed ID in closest location: {closest_seed}");
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

struct Map {
    from_name: String,
    to_name: String,
    ranges: Vec<MapRange>,
}

struct MapRange {
    dest_id: usize,
    source_id: usize,
    range_length: usize,
}

impl Almanac {
    fn from_file<I>(lines: I) -> Option<Almanac>
    where
        I: IntoIterator,
        I::Item: Borrow<str>,
    {
        let mut lines = lines.into_iter();
        let seeds = parse_seeds_line(lines.next().unwrap().borrow())?;
        lines.next()?;
        let mut maps = Vec::new();
        let mut map: Option<Map> = None;
        for line in lines {
            let line = line.borrow().trim();
            if line.is_empty() && map.is_some() {
                maps.push(map.take().unwrap());  // blank lines separate mappings
                continue;
            }
            if let Some(map) = &mut map {
                // <dest> <source> <N>
                map.ranges.push(MapRange::from_str(line)?);
            } else {
                if map.is_some() {
                    panic!("Line out of sequence, expect new map.")
                }
                map = Some(Map::from_str(line)?);
            }
        }
        if map.is_some() {
            maps.push(map.take().unwrap());
        }

        Some(Almanac{seeds, maps})
    }

    fn map_final(&self, id: usize) -> usize {
        self.maps.iter().fold(id, |acc, m| m.map_from(acc))
    }

    fn min_loc(&self) -> usize {
        self.seeds.iter().map(|s| self.map_final(*s)).min().unwrap()
    }
}

fn parse_seeds_line(line: &str) -> Option<Vec<usize>> {
    lazy_static! {
        static ref SEED_LINE_RE: Regex = Regex::new(r"^seeds: *([\d ]+)$").unwrap();
    }
    let m = SEED_LINE_RE.captures(line)?;
    Some(utils::parse_numbers_from_str(m.get(1).unwrap().as_str()))
}

impl Map {
    fn from_str(s: &str) -> Option<Map> {
        lazy_static! {
            static ref MAP_LINE_RE: Regex = Regex::new(r"^(\w+)-to-(\w+) map:$").unwrap();
        }
        let m = MAP_LINE_RE.captures(s)?;
        Some(Map{
            from_name: m.get(1).unwrap().as_str().to_owned(),
            to_name: m.get(2).unwrap().as_str().to_owned(),
            ranges: Vec::new(),
        })
    }

    fn map_from(&self, from_id: usize) -> usize {
        for map in self.ranges.iter() {
            if let Some(to_id) = map.map_from(from_id) {
                return to_id;
            }
        }
        from_id
    }
}

impl MapRange {
    fn from_str(s: &str) -> Option<MapRange> {
        let nums = utils::parse_numbers_from_str(s);
        if nums.len() != 3 {
            None
        } else {
            Some(MapRange{
                dest_id: nums[0],
                source_id: nums[1],
                range_length: nums[2],
            })
        }
    }

    fn map_from(&self, from_id: usize) -> Option<usize> {
        if from_id < self.source_id {
            return None
        }
        let pos = from_id - self.source_id;
        if pos >= self.range_length {
            None
        } else {
            Some(self.dest_id + pos)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    const EXAMPLE_1: &str = r#"seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4"#;

    fn parse_example() -> Almanac {
        Almanac::from_file(EXAMPLE_1.lines()).unwrap()
    }

    #[test]
    fn test_parse_example() {
        let a = parse_example();
        assert_eq!(a.seeds.len(), 4);
        assert_eq!(a.maps.len(), 7);
        assert_eq!(a.maps[0].from_name, "seed");
        assert_eq!(a.maps[6].to_name, "location");
    }

    #[test]
    fn test_map_final() {
        let a = parse_example();
        for (seed_id, expected) in zip(&a.seeds, vec![82, 43, 86, 35]) {
            let loc = a.map_final(*seed_id);
            assert_eq!(loc, expected);
        }
        assert_eq!(a.min_loc(), 35);
    }
}