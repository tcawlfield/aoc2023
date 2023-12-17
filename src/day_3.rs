// use std::borrow::Borrow;
// use std::cmp;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT: &str = "inputs/input_3.txt";

pub fn main() {
    let file = File::open(INPUT).unwrap();
    let mut lines = Vec::new();
    for (row, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        let parsed = Line::from_str(row, &line);
        if let Some(line) = parsed {
            lines.push(line);
        } else {
            println!("Unparsed line: {line:?}");
        }
    }

    let total = total_part_numbers(&lines);
    println!("Pt 1: Total of part numbers: {total}");

    let total_2 = total_gear_ratios(&lines);
    println!("Pt 2: Total of gear ratios: {total_2}");
}

fn total_part_numbers(lines: &Vec<Line>) -> usize {
    let mut total = 0;
    let empty = Line::empty();
    for sym_idx in 0..lines.len() {
        let prev = if sym_idx > 0 {
            &lines[sym_idx - 1]
        } else {
            &empty
        };
        let cur = &lines[sym_idx];
        let next = if sym_idx < lines.len() - 1 {
            &lines[sym_idx + 1]
        } else {
            &empty
        };
        for num in cur.numbers.iter() {
            for sym in prev
                .symbols
                .iter()
                .chain(cur.symbols.iter())
                .chain(next.symbols.iter())
            {
                if sym.column + 1 >= num.first_col && sym.column <= num.last_col + 1 {
                    total += num.number;
                }
            }
        }
    }
    total
}

fn total_gear_ratios(lines: &Vec<Line>) -> usize {
    let mut total = 0;
    let empty = Line::empty();
    for sym_idx in 0..lines.len() {
        let prev = if sym_idx > 0 {
            &lines[sym_idx - 1]
        } else {
            &empty
        };
        let cur = &lines[sym_idx];
        let next = if sym_idx < lines.len() - 1 {
            &lines[sym_idx + 1]
        } else {
            &empty
        };
        let mut number_neighbors = Vec::new();
        for sym in cur.symbols.iter() {
            if sym.symbol != '*' {
                continue;
            }
            number_neighbors.clear();
            for num in prev
                .numbers
                .iter()
                .chain(cur.numbers.iter())
                .chain(next.numbers.iter())
            {
                if sym.column + 1 >= num.first_col && sym.column <= num.last_col + 1 {
                    number_neighbors.push(num.number);
                }
            }
            if number_neighbors.len() == 2 {
                total += number_neighbors[0] * number_neighbors[1];
            }
        }
    }
    total
}

#[allow(dead_code)]
struct Line {
    row: usize,
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

#[allow(dead_code)]
struct Symbol {
    symbol: char,
    column: usize,
}

#[allow(dead_code)]
struct Number {
    number: usize,
    first_col: usize,
    last_col: usize,
}

impl Line {
    fn empty() -> Line {
        Line {
            row: 0,
            symbols: Vec::new(),
            numbers: Vec::new(),
        }
    }

    fn from_str(row: usize, s: &str) -> Option<Line> {
        lazy_static! {
            static ref OK_LINE_RE: Regex = Regex::new(r"\.").unwrap();
        }
        if !OK_LINE_RE.is_match(s) {
            return None;
        }
        let s = s.trim();
        let symbols = Symbol::from_line(s);
        let numbers = Number::from_line(s);
        Some(Line {
            row,
            symbols,
            numbers,
        })
    }
}

impl Symbol {
    fn from_line(line: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for (column, ch) in line.chars().enumerate() {
            let sym = match ch {
                '.' => None,
                _ if ch.is_alphanumeric() => None,
                _ => Some(ch),
            };
            if let Some(symbol) = sym {
                symbols.push(Symbol { symbol, column });
            }
        }
        symbols
    }
}

impl Number {
    fn from_line(line: &str) -> Vec<Number> {
        lazy_static! {
            static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
        }

        let mut numbers = Vec::new();
        for m in NUM_RE.find_iter(line) {
            numbers.push(Number {
                number: usize::from_str_radix(m.as_str(), 10).unwrap(),
                first_col: m.start(),
                last_col: m.end() - 1,
            });
        }
        numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = r#"467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."#;

    fn sample_lines() -> Vec<Line> {
        EXAMPLE_1
            .lines()
            .enumerate()
            .map(|(i, s)| Line::from_str(i, s))
            .filter_map(std::convert::identity)
            .collect()
    }

    #[test]
    fn test_line_read() {
        let lines = sample_lines();
        assert_eq!(lines[0].numbers.len(), 2);
        assert_eq!(lines[0].symbols.len(), 0);
        assert_eq!(lines[1].numbers.len(), 0);
        assert_eq!(lines[1].symbols.len(), 1);
    }

    #[test]
    fn test_pt_1() {
        let lines = sample_lines();
        let total = total_part_numbers(&lines);
        assert_eq!(total, 4361);
    }

    #[test]
    fn test_pt_2() {
        let lines = sample_lines();
        let total = total_gear_ratios(&lines);
        assert_eq!(total, 467835);
    }
}
