use std::fs::File;
use std::io::{self, BufRead};

const INPUT: &str = "inputs/input_6.txt";

pub fn main() {
    let file = File::open(INPUT).unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().take(2).flatten().collect();
    let times = line_to_ints(&lines[0]);
    let distances = line_to_ints(&lines[1]);
    let ways: Vec<i64> = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| ways_to_beat(*t, *d))
        .collect();
    let ttl_ways: i64 = ways.iter().product();
    println!("Part 1: {ttl_ways} total ways to beat all records");

    let one_time = line_to_single_int(&lines[0]);
    let one_dist = line_to_single_int(&lines[1]);
    println!("Time: {one_time}, Record: {one_dist}");
    let ttl_ways = ways_to_beat(one_time, one_dist);
    println!("Part 2: {ttl_ways} total ways to win the one race.");
}

fn line_to_ints(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn line_to_single_int(line: &str) -> i64 {
    let pieces: Vec<&str> = line.split_whitespace().skip(1).collect();
    pieces.join("").parse().unwrap()
}

/*   Math:
 * Hold down charging button for $c$ millis.
 * Total time available is $t$.
 * Velocity of boat: $v = c$.
 * Distance boat travels: $v (t - c) = c (t - c) = ct - c^2$
 * To beat the record r, $ct - c^2 > r$ which lies inside the interval...
 * (-b±√(b²-4ac))/(2a) with a -> 1, b -> -t, c -> r. So:
 * ( t ± √(t^2 - 4r) ) / 2.
 */

enum Bounds {
    Impossible,
    JustPossible(f64),
    Within(f64, f64),
}

fn times_to_match(t: f64, r: f64) -> Bounds {
    let pms = t * t - 4.0 * r;
    if pms < 0.0 {
        return Bounds::Impossible;
    } else if pms == 0.0 {
        return Bounds::JustPossible(t / 2.0);
    } else {
        let pm = pms.sqrt();
        return Bounds::Within((t - pm) / 2.0, (t + pm) / 2.0);
    }
}

fn ways_to_beat(duration: i64, record: i64) -> i64 {
    let ttm = times_to_match(duration as f64, record as f64);
    match ttm {
        Bounds::Impossible => 0,
        Bounds::JustPossible(_) => 0,
        Bounds::Within(low, high) => {
            println!("Can beat {duration} ms with record {record} mm between {low} and {high}");
            high.ceil() as i64 - low.floor() as i64 - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ways() {
        assert_eq!(ways_to_beat(7, 9), 4);
        assert_eq!(ways_to_beat(15, 40), 8);
        assert_eq!(ways_to_beat(30, 200), 9);
    }

    #[test]
    fn test_line_to_single() {
        let lines = vec!["Time:      7  15   30", "Distance:  9  40  200"];
        let one_time = line_to_single_int(&lines[0]);
        let one_dist = line_to_single_int(&lines[1]);
        let ttl_ways = ways_to_beat(one_time, one_dist);
        assert_eq!(one_time, 71530);
        assert_eq!(one_dist, 940200);
        assert_eq!(ttl_ways, 71503);
    }
}
