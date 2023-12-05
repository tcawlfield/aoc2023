use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_input("input/input_1.txt").unwrap();
    let cal_sum: u32 = lines
        .filter_map(|line| line.ok())
        .map(|line| calibration_value(line.trim()))
        .sum();
    println!("Part 1: Sum of calibration values is {}", cal_sum);
}

fn calibration_value(line: &str) -> u32 {
    let as_chars: Vec<_> = line.chars().collect();
    0
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
}
