use std::fs::File;
use std::io::{self, BufRead};

pub fn lines_in_file(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Bad line!"))
}

pub fn parse_numbers_from_str(s: &str) -> Vec<usize> {
    s.trim()
        .split(' ')
        .filter_map(|s| {
            if s.len() > 0 {
                Some(s.parse().unwrap())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_from_str() {
        let numbers = parse_numbers_from_str(" 1 12  13   145 ");
        assert_eq!(numbers, vec![1, 12, 13, 145]);
    }
}
