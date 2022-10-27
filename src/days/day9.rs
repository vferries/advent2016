use std::ops::Index;
use std::str::Lines;

use itertools::Itertools;

pub fn day9() {
    println!("=== Day 9 ===");
    println!("- Part 1");
    let line = parse_file();
    println!("{}", part1(line));
    println!("- Part 2");
    let line = parse_file();
    part2(line);
}

pub fn part1(str: &str) -> usize {
    decompress(str).len()
}

pub fn part2(str: &str) -> usize {
    decompress_v2(str)
}

fn parse_file() -> &'static str {
    include_str!("input/day9.txt")
}

fn decompress(str: &str) -> String {
    let mut i = 0;
    let mut result = String::new();
    while i < str.len() {
        if char_at(str, i) == '(' {
            let start = i;
            while char_at(str, i) != ')' {
                i += 1;
            }
            let end = i;
            let marker = &str[(start + 1)..end];
            let (nb_chars, count) = marker.split_once('x').unwrap();
            let nb_chars: usize = nb_chars.parse().unwrap();
            let count: usize = count.parse().unwrap();
            result += &str[(end + 1)..(end + nb_chars + 1)].repeat(count);
            i += nb_chars + 1;
        } else {
            result += char_at(str, i).to_string().as_str();
            i += 1;
        }
    }
    result
}

fn decompress_v2(str: &str) -> usize {
    let mut i = 0;
    let mut result = 0;
    while i < str.len() {
        if char_at(str, i) == '(' {
            let start = i;
            while char_at(str, i) != ')' {
                i += 1;
            }
            let end = i;
            let marker = &str[(start + 1)..end];
            let (nb_chars, count) = marker.split_once('x').unwrap();
            let nb_chars: usize = nb_chars.parse().unwrap();
            let count: usize = count.parse().unwrap();
            result += count * decompress_v2(&str[(end + 1)..(end + nb_chars + 1)]);
            i += nb_chars + 1;
        } else {
            result += 1;
            i += 1;
        }
    }
    result
}

fn char_at(str: &str, mut i: usize) -> char {
    str.chars().nth(i).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(102239, part1(include_str!("input/day9.txt")));
    }

    #[test]
    fn part1_advent() {
        assert_eq!("ADVENT", decompress("ADVENT"));
    }

    #[test]
    fn part1_repeat_one() {
        assert_eq!("ABBBBBC", decompress("A(1x5)BC"));
    }

    #[test]
    fn part1_repeat_multiple_chars() {
        assert_eq!("XYZXYZXYZ", decompress("(3x3)XYZ"));
    }

    #[test]
    fn part1_repeat_multiple_times() {
        assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG"));
    }

    #[test]
    fn part1_repeat_marker() {
        assert_eq!("(1x3)A", decompress("(6x1)(1x3)A"));
    }

    #[test]
    fn part1_repeat_complex() {
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn part2_sample1() {
        assert_eq!(9, decompress_v2("(3x3)XYZ"));
    }

    #[test]
    fn part2_sample2() {
        assert_eq!(20, decompress_v2("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn part2_sample3() {
        assert_eq!(241920, decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
    }

    #[test]
    fn part2_sample4() {
        assert_eq!(445, decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));
    }

    #[test]
    fn part2_test() {
        assert_eq!(10780403063, part2(include_str!("input/day9.txt")));
    }
}
