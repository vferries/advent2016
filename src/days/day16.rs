use std::ops::Index;

use fancy_regex::Regex;
use itertools::Itertools;

pub fn day16() {
    let input = "10111011111001111";
    let result = part1(input, 272);
    println!("Part 1 = {}", result);
    let result = part1(input, 35651584);
    println!("Part 2 = {}", result);
}

fn part1(input: &str, disk_size: usize) -> String {
    let curve = generate_dragon_curve(String::from(input), disk_size);
    checksum(curve)
}

fn generate_dragon_curve(pattern: String, size: usize) -> String {
    return if pattern.len() >= size {
        String::from(&pattern[0..size])
    } else {
        let next = format!("{}0{}", pattern.clone(), String::from(pattern).reversed());
        generate_dragon_curve(next, size)
    };
}

trait Reversible {
    fn reversed(self: &Self) -> String;
}

impl Reversible for String {
    fn reversed(self: &Self) -> String {
        self.chars().collect_vec().iter().rev().map(|c| {
            if *c == '0' { '1' } else { '0' }
        }).join("")
    }
}

fn checksum(input: String) -> String {
    return if input.len() % 2 == 0 {
        checksum(input.chars().chunks(2).into_iter().map(|c| {
            let (a, b) = c.collect_tuple().unwrap();
            if a == b { '1' } else { '0' }
        }).join(""))
    } else {
        input
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!("01100", part1("10000", 20));
    }

    #[test]
    fn part1_input() {
        assert_eq!("", part1("10111011111001111", 272));
    }
}
