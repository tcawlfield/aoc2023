use std::fs::File;
use std::io::{self, BufRead};

const INPUT: &str = "inputs/input_6.txt";

pub fn main() {
    let file = File::open(INPUT).unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().take(2).flatten().collect();
    let times = line_to_ints(&lines[0]);
    let distances = line_to_ints(&lines[1]);
    let ways_to_beat: Vec<i32> = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| ways_to_beat(*t, *d))
        .collect();
    let ttl_ways: i32 = ways_to_beat.iter().product();
    println!("Part 1: {ttl_ways} total ways to beat all records");
}

fn line_to_ints(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
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
    JustPossible(f32),
    Within(f32, f32),
}

fn times_to_match(t: f32, r: f32) -> Bounds {
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

fn ways_to_beat(duration: i32, record: i32) -> i32 {
    let ttm = times_to_match(duration as f32, record as f32);
    match ttm {
        Bounds::Impossible => 0,
        Bounds::JustPossible(_) => 0,
        Bounds::Within(low, high) => {
            // println!("Can beat {duration} ms with record {record} mm between {low} and {high}");
            high.ceil() as i32 - low.floor() as i32 - 1
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
}
