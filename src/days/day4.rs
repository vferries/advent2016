use std::str::Lines;

use itertools::Itertools;
use regex::Regex;

pub fn day4() {
    println!("=== Day 4 ===");
    println!("- Part 1");
    let lines = parse_file();
    println!("{}", part1(lines));
    println!("- Part 2");
    let lines = parse_file();
    println!("{}", part2(lines));
}

pub fn part1(lines: Lines) -> usize {
    lines
        .map(parse_line)
        .filter(|(name, _, checksum)| real_room(name, checksum))
        .map(|(_, sector_id, _)| sector_id)
        .sum()
}

pub fn part2(lines: Lines) -> usize {
    lines
        .map(parse_line)
        .filter(|(name, _, checksum)| real_room(name, checksum))
        .map(|(name, sector_id, _)| (decrypt(&name, sector_id), sector_id))
        .find_or_first(|(name, _)| name.contains("northpole"))
        .unwrap()
        .1
}

fn decrypt(name: &str, sector_id: usize) -> String {
    name.chars()
        .map(|c| match c {
            '-' => ' ',
            c => rotate(c, sector_id),
        })
        .join("")
}

fn rotate(c: char, count: usize) -> char {
    let letters: Vec<char> = ('a'..='z').into_iter().collect();
    let index = letters.binary_search(&c).unwrap();
    let index = (index + count) % 26;
    letters[index]
}

fn real_room(room_name: &str, checksum: &str) -> bool {
    let room_name = room_name.replace("-", "");
    calculate_checksum(&room_name) == checksum
}

fn parse_line(line: &str) -> (String, usize, String) {
    let re = Regex::new(r"^(.*)-(\d+)\[(.*)]$").unwrap();
    let cap = re.captures(line).unwrap();
    let letters = String::from(&cap[1]);
    let sector_id = cap[2].parse().unwrap();
    let checksum = String::from(&cap[3]);
    (letters, sector_id, checksum)
}

fn calculate_checksum(letters: &str) -> String {
    let counts = letters.chars().counts_by(|l| l);
    counts
        .iter()
        .sorted_by(|(&c1, &s1), (&c2, &s2)| s2.cmp(&s1).then(c1.cmp(&c2)))
        .take(5)
        .map(|(k, _)| k)
        .join("")
}

fn parse_file() -> Lines<'static> {
    include_str!("input/day4.txt").lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(true, real_room("aaaaa-bbb-z-y-x", "abxyz"));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!(true, real_room("a-b-c-d-e-f-g-h", "abcde"));
    }

    #[test]
    fn part1_sample3() {
        assert_eq!(true, real_room("not-a-real-room", "oarel"));
    }

    #[test]
    fn part1_sample4() {
        assert_eq!(false, real_room("totally-real-room", "decoy"));
    }

    #[test]
    fn part1_complete() {
        assert_eq!(245102, part1(parse_file()));
    }

    #[test]
    fn decrypt_sample() {
        assert_eq!("very encrypted name", decrypt("qzmt-zixmtkozy-ivhz", 343));
    }
}
