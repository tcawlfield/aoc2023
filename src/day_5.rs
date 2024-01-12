use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::mem;

const INPUT: &str = "inputs/input_5.txt";

pub fn main() {
    let almanac = Almanac::from_file(utils::lines_in_file(INPUT)).unwrap();
    let closest_seed = almanac.min_loc();
    println!("Day 5 pt 1: Seed ID in closest location: {closest_seed}");

    let pt2_closest_seed = almanac.min_loc_ranges();
    println!("Day 5 pt 2: Seed ID in closest location: {pt2_closest_seed}");
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

#[allow(dead_code)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
struct IDRange {
    map_level: usize, // 0 = seed, 1 = soil, etc.
    start_id: usize,
    length: usize,
    // sub_ranges: Option<Vec<IDRange>>,
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
                maps.push(map.take().unwrap()); // blank lines separate mappings
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

        Some(Almanac { seeds, maps })
    }

    fn map_final(&self, id: usize) -> usize {
        self.maps.iter().fold(id, |acc, m| m.map_from(acc))
    }

    fn min_loc(&self) -> usize {
        self.seeds.iter().map(|s| self.map_final(*s)).min().unwrap()
    }

    fn min_loc_ranges(&self) -> usize {
        let mapped_ranges = self.map_final_ranges();
        mapped_ranges.iter().map(|r| r.start_id).min().unwrap()
    }

    fn map_final_ranges(&self) -> Vec<IDRange> {
        let ranges_in = self.seeds_as_ranges();
        let mut ranges_in = Box::new(ranges_in);
        for map in self.maps.iter() {
            let mapped_ranges = map.find_map_ranges(&ranges_in);
            *ranges_in = mapped_ranges;
        }
        *ranges_in
    }

    fn seeds_as_ranges(&self) -> Vec<IDRange> {
        // Interpret seeds as a list of pairs: (start_id, length)
        let mut ranges = Vec::new();
        for pair in self.seeds[..].chunks(2) {
            ranges.push(IDRange::new(0, pair[0], pair[1]));
        }
        ranges
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
        Some(Map {
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

    fn find_map_ranges(&self, ranges: &[IDRange]) -> Vec<IDRange> {
        let mut mapped_ranges = Vec::new();
        let mut unmapped_ranges = ranges.to_vec(); // copy
        let this_level = ranges[0].map_level;
        let mut new_ranges = Vec::new();
        for map in self.ranges.iter() {
            // Loop over this Map's MapRange's
            new_ranges.clear();
            for id_range in unmapped_ranges.iter() {
                // Loop over input or leftover, unmapped ranges
                let potential_ranges = map.map_range(&id_range);
                for potential in potential_ranges {
                    // Loop over the up-to-3 results
                    if let Some(id_range) = potential {
                        if id_range.map_level > this_level {
                            // Taken care of by this mapping, so it's all done.
                            mapped_ranges.push(id_range);
                        } else {
                            // Need to try next level
                            new_ranges.push(id_range);
                        }
                    }
                }
            }
            // Next mapping gets all our would-be-defaulted ranges.
            mem::swap(&mut new_ranges, &mut unmapped_ranges);
        }
        // Every range still-unmapped defaults to original IDs
        for mut unmapped_range in unmapped_ranges {
            unmapped_range.map_level += 1;
            mapped_ranges.push(unmapped_range);
        }
        mapped_ranges
    }
}

impl MapRange {
    fn from_str(s: &str) -> Option<MapRange> {
        let nums = utils::parse_numbers_from_str(s);
        if nums.len() != 3 {
            None
        } else {
            Some(MapRange {
                dest_id: nums[0],
                source_id: nums[1],
                range_length: nums[2],
            })
        }
    }

    fn map_from(&self, from_id: usize) -> Option<usize> {
        if from_id < self.source_id {
            return None;
        }
        let pos = from_id - self.source_id;
        if pos >= self.range_length {
            None
        } else {
            Some(self.dest_id + pos)
        }
    }

    fn last_source_id(&self) -> usize {
        self.source_id + self.range_length - 1
    }

    fn map_range(&self, source_range: &IDRange) -> [Option<IDRange>; 3] {
        // This takes a (contiguous) source ID range and maps it into up-to three ranges.
        // Parts of the source range outside the mapped range will retain a map_level equal
        // to the given source_range's level to indicate that no explicit mapping occured.
        // That part of the source range within the mapping interval corresponds to
        // the (second) element of the output being mapped with incremented map_level.

        // Note to self: this approach feels too complicated to me. I'm still struggling
        // to find a way to simplify it.

        let mut mapped = [None; 3];
        if (source_range.last() < self.source_id) || (source_range.start_id > self.last_source_id())
        {
            // Easy -- source range is entirely outside our mapping range
            mapped[0] = Some(source_range.clone());
            return mapped;
            // It's nice to take care of this case early so that later on we can assume
            // that some part of source_range is explicitly mapped.
        }

        let source_range = if source_range.start_id < self.source_id {
            // Some of source_range is before the mapping.
            let head_length = self.source_id - source_range.start_id;
            mapped[0] = Some(IDRange {
                map_level: source_range.map_level, // not explicitly mapped
                start_id: source_range.start_id,
                length: head_length,
            });
            IDRange {
                map_level: source_range.map_level,
                start_id: self.source_id,
                length: source_range.length - head_length,
            }
        } else {
            source_range.clone()
        };
        if source_range.last() > self.last_source_id() {
            // Source range extends beyond this mapping.
            mapped[1] = Some(IDRange {
                map_level: source_range.map_level + 1, // Explicitly-mapped region -- advanced map_level to indicate this.
                start_id: self.map_from(source_range.start_id).unwrap(),
                length: self.last_source_id() - source_range.start_id + 1,
            });
            mapped[2] = Some(IDRange {
                map_level: source_range.map_level, // not explicitly mapped
                start_id: self.last_source_id() + 1,
                length: source_range.last() - self.last_source_id(),
            });
        } else {
            // Source range lies within this mapping.
            mapped[1] = Some(IDRange {
                map_level: source_range.map_level + 1, // Explicitly-mapped region -- advanced map_level to indicate this.
                start_id: self.map_from(source_range.start_id).unwrap(),
                length: source_range.length,
            });
        }
        mapped
    }
}

impl IDRange {
    fn new(map_level: usize, start_id: usize, length: usize) -> IDRange {
        IDRange {
            map_level,
            start_id,
            length,
        }
    }

    //     fn map_and_subdivide(&mut self, maps: &[Map]) {
    //         if self.map_level >= maps.len() {
    //             return;
    //         }
    //         let map = &maps[self.map_level];
    //         let mut subranges = Vec::new();
    //
    //         subranges.extend(map.find_map_ranges(&self).iter());
    //
    //         for subrange in subranges {
    //             subrange.map_and_subdivide(maps);
    //         }
    //         self.sub_ranges = Some(subranges);
    //     }

    fn last(&self) -> usize {
        self.start_id + self.length - 1
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

    #[test]
    fn check_map_range_map_range() {
        let mr = MapRange::from_str("100 10 5").unwrap();
        let to_mapped = IDRange {
            map_level: 1,
            start_id: 1,
            length: 2,
        };
        assert_eq!(mr.map_range(&to_mapped), [Some(to_mapped), None, None]);
        let to_mapped = IDRange {
            map_level: 1,
            start_id: 15,
            length: 2,
        };
        assert_eq!(mr.map_range(&to_mapped), [Some(to_mapped), None, None]);

        let to_mapped = IDRange {
            map_level: 1,
            start_id: 5,
            length: 15,
        };
        assert_eq!(
            mr.map_range(&to_mapped),
            [
                Some(IDRange {
                    map_level: 1,
                    start_id: 5,
                    length: 5
                }),
                Some(IDRange {
                    map_level: 2,
                    start_id: 100,
                    length: 5
                }),
                Some(IDRange {
                    map_level: 1,
                    start_id: 15,
                    length: 5
                }),
            ]
        );

        let to_mapped = IDRange {
            map_level: 1,
            start_id: 5,
            length: 8,
        };
        assert_eq!(
            mr.map_range(&to_mapped),
            [
                Some(IDRange {
                    map_level: 1,
                    start_id: 5,
                    length: 5
                }),
                Some(IDRange {
                    map_level: 2,
                    start_id: 100,
                    length: 3
                }),
                None,
            ]
        );

        let to_mapped = IDRange {
            map_level: 1,
            start_id: 12,
            length: 8,
        };
        assert_eq!(
            mr.map_range(&to_mapped),
            [
                None,
                Some(IDRange {
                    map_level: 2,
                    start_id: 102,
                    length: 3
                }),
                Some(IDRange {
                    map_level: 1,
                    start_id: 15,
                    length: 5
                }),
            ]
        );
    }

    #[test]
    fn test_map_range_final() {
        let a = parse_example();
        let min_loc = a.min_loc_ranges();
        assert_eq!(min_loc, 46);
    }
}
