use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::Lines;

pub fn day2() {
    println!("=== Day 2 ===");
    println!("- Part 1");
    let lines = parse_file();
    println!("{}", part1(lines));

    println!("- Part 2");
    let lines = parse_file();
    println!("{}", part2(lines));
}

fn part1(lines: Lines) -> i32 {
    let mut result = 0;
    let (mut x, mut y) = (1, 1);
    for line in lines {
        for c in line.chars() {
            (x, y) = match c {
                'U' => (x, max(0, y - 1)),
                'D' => (x, min(2, y + 1)),
                'L' => (max(0, x - 1), y),
                'R' => (min(2, x + 1), y),
                _ => panic!("Unexpected character !")
            }
        }
        result *= 10;
        result += 3 * y + x + 1;
    }
    result
}

fn part2(lines: Lines) -> String {
    let pad = create_pad();
    let mut result = String::from("");
    let (mut x, mut y) = (0, 2);
    for line in lines {
        for c in line.chars() {
            let next = match c {
                'U' => (x, if y > 0 { y - 1 } else { 0 }),
                'D' => (x, y + 1),
                'L' => (if x > 0 { x - 1 } else { 0 }, y),
                'R' => (x + 1, y),
                _ => panic!("Unknown direction")
            };
            (x, y) = match pad.get(&next) {
                Some(_) => next,
                None => (x, y),
            };
        }
        result.push(*pad.get(&(x, y)).unwrap());
    }
    result
}

fn create_pad() -> HashMap<(usize, usize), char> {
    let mut map = HashMap::new();
    let pattern = include_str!("input/day2_pattern.txt").lines();
    for (y, line) in pattern.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != ' ' {
                map.insert((x, y), c);
            }
        }
    }
    map
}

fn parse_file() -> Lines<'static> {
    include_str!("input/day2.txt").lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let result = part1(include_str!("input/day2_sample.txt").lines());
        assert_eq!(1985, result);
    }

    #[test]
    fn test_part1() {
        let result = part1(parse_file());
        assert_eq!(84452, result);
    }

    #[test]
    fn sample_part2() {
        let result = part2(include_str!("input/day2_sample.txt").lines());
        assert_eq!("5DB3", result);
    }

    #[test]
    fn test_part2() {
        let result = part2(parse_file());
        assert_eq!("D65C3", result);
    }
}
