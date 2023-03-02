use std::cmp::{max, Ordering};

pub fn day20() {
    let ranges = parse_file(include_str!("input/day20.txt"));
    let result = part1(ranges);
    println!("Part 1 = {}", result);
    let ranges = parse_file(include_str!("input/day20.txt"));
    let result = part2(ranges);
    println!("Part 2 = {}", result);
}

#[derive(Debug, Copy, Clone)]
struct UnsignedRange {
    start: u32,
    end: u32,
}

fn parse_file(file_content: &str) -> Vec<UnsignedRange> {
    file_content.lines().map(|line| {
        let nums: Vec<_> = line.split("-").collect();
        let left: u32 = nums[0].parse().unwrap();
        let right: u32 = nums[1].parse().unwrap();
        UnsignedRange { start: left, end: right }
    }
    ).collect()
}

fn part1(mut ranges: Vec<UnsignedRange>) -> u32 {
    ranges.sort();
    let mut max_range = ranges[0];
    for i in 1..ranges.len() {
        if ranges[i].start > max_range.end + 1 {
            break
        } else {
            max_range.end = max(ranges[i].end, max_range.end);
        }
    }
    return max_range.end + 1
}

fn part2(mut ranges: Vec<UnsignedRange>) -> u32 {
    ranges.sort();

    let mut i = 0;
    while i < ranges.len() - 1 {
        let r1 = ranges[i];
        let r2 = ranges[i+1];
        if r2.start > r1.end {
            i += 1;
        } else {
            ranges[i].end = max(r1.end, r2.end);
            ranges.remove(i+1);
        }
    }
    let mut sum = 0;
    for i in 1..ranges.len() {
        sum += ranges[i].start - ranges[i-1].end - 1;
    }
    return sum;
}

impl PartialOrd for UnsignedRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for UnsignedRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialEq<Self> for UnsignedRange {
    fn eq(&self, other: &Self) -> bool {
        self.start.eq(&other.start)
    }
}

impl Eq for UnsignedRange {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let ranges = parse_file(include_str!("input/day20_sample.txt"));
        assert_eq!(3, part1(ranges));
    }

    #[test]
    fn part2_sample() {
        let ranges = parse_file(include_str!("input/day20_sample.txt"));
        assert_eq!(4294967287, part2(ranges));
    }
}
