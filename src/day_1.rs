use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    {
        let lines = read_input("inputs/input_1.txt").unwrap();
        let cal_sum: u32 = lines
            .filter_map(|line| line.ok())
            .map(|line| calibration_value(line.trim()))
            .sum();
        println!("Part 1: Sum of calibration values is {}", cal_sum);
    }
    {
        let lines = read_input("inputs/input_1.txt").unwrap();
        let cal_sum: u32 = lines
            .filter_map(|line| line.ok())
            .map(|line| calibration_value_with_spelling(line.trim()))
            .sum();
        println!("Part 1: Sum of calibration values is {}", cal_sum);
    }
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calibration_value(line: &str) -> u32 {
    let tens = first_digit(line.chars());
    let ones = first_digit(line.chars().rev());
    tens * 10 + ones
}

fn first_digit<T>(seq: T) -> u32
where
    T: Iterator<Item = char>,
{
    seq.filter(|c| c.is_ascii_digit())
        .nth(0)
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn calibration_value_with_spelling(line: &str) -> u32 {
    let line_unspelled = replace_digit_names(line);
    let tens = first_digit(line_unspelled.chars());
    let ones = first_digit(line_unspelled.chars().rev());
    tens * 10 + ones
}

// replace_digit_names  returns a copy of the given string, with the first letter of
// each spelled-out digit replaced with the numerical digit.
fn replace_digit_names(line: &str) -> String {
    lazy_static! {
        static ref DIGIT_MAP: HashMap<&'static str, char> = HashMap::from([
            ("zero", '0'),
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);
        static ref RE: Regex =
            Regex::new(&DIGIT_MAP.keys().map(|s| &**s).collect::<Vec<_>>().join("|")).unwrap();
    }
    let mut newbytes = line.to_owned().into_bytes();
    let mut start_pos = 0;
    // We cannot simply loop over re.find_iter, since these only report *non-overlapping* matches.
    // I want to allow for overlapping matches, because ... well ... I just do!
    // The given example "zoneight234" in my version translates to "z1n8ight234".
    // With or without overlaps, this results in the calibration value 14.
    loop {
        let substr = &line[start_pos..];
        if let Some(mtch) = RE.find_iter(&substr).next() {
            let digit = DIGIT_MAP.get(mtch.as_str()).unwrap();
            start_pos += mtch.start();
            newbytes[start_pos] = *digit as u8;
            start_pos += 1; // Moving on...
        } else {
            break;
        }
    }
    String::from_utf8(newbytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_pt1() {
        assert_eq!(calibration_value("1abc2"), 12);
        assert_eq!(calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn test_replace_digit_names() {
        assert_eq!(replace_digit_names("fooneight"), "fo1n8ight");
    }

    #[test]
    fn test_day_1_pt2() {
        assert_eq!(calibration_value_with_spelling("two1nine"), 29);
        assert_eq!(calibration_value_with_spelling("eightwothree"), 83);
        assert_eq!(calibration_value_with_spelling("abcone2threexyz"), 13);
        assert_eq!(calibration_value_with_spelling("xtwone3four"), 24);
        assert_eq!(calibration_value_with_spelling("4nineeightseven2"), 42);
        assert_eq!(calibration_value_with_spelling("zoneight234"), 14);
        assert_eq!(calibration_value_with_spelling("7pqrstsixteen"), 76);
    }
}
