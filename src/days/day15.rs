use std::ops::Index;

use fancy_regex::Regex;
use itertools::Itertools;

pub fn day15() {
    let result = part1(include_str!("input/day15.txt"));
    println!("Part 1 = {}", result);
    let result = part2(include_str!("input/day15.txt"));
    println!("Part 2 = {}", result);
}

fn part1(input: &str) -> usize {
    let discs = parse_discs(input);
    first_aligned_time(discs)
}

fn part2(input: &str) -> usize {
    let mut discs = parse_discs(input);
    let additional_disc = Disc { index: discs.len() + 1, positions: 11, init: 0 };
    discs.push(additional_disc);
    first_aligned_time(discs)
}

fn parse_discs(input: &str) -> Vec<Disc> {
    let discs = input.lines().map(|line| {
        Disc::new(line)
    }).collect_vec();
    discs
}

fn first_aligned_time(discs: Vec<Disc>) -> usize {
    let mut t = 0;
    while discs.iter().any(|d| !d.aligned(t)) {
        t += 1;
    }
    t
}

#[derive(Debug)]
struct Disc {
    index: usize,
    positions: usize,
    init: usize,
}

impl Disc {
    fn aligned(&self, t: usize) -> bool {
        (t + self.init + self.index) % self.positions == 0
    }
}

impl Disc {
    fn new(str: &str) -> Self {
        let regex = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
        let captures = regex.captures(str).unwrap().unwrap();
        let index = captures.index(1).parse().unwrap();
        let positions = captures.index(2).parse().unwrap();
        let init = captures.index(3).parse().unwrap();
        Disc { index, positions, init }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(5, part1(include_str!("input/day15_sample.txt")));
    }

    #[test]
    fn part1_input() {
        assert_eq!(400589, part1(include_str!("input/day15.txt")));
    }
}
