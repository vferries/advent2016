use std::collections::{HashMap, VecDeque};
use std::ops::Index;
use std::str::Lines;

use fancy_regex::Regex;

use crate::days::day10::StopCondition::{CompareValues, ProcessAll};

pub fn day10() {
    println!("=== Day 10 ===");
    println!("- Part 1");
    let line = parse_file();
    println!("{}", part1(line));
    println!("- Part 2");
    let line = parse_file();
    println!("{}", part2(line));
}

pub fn part1(instructions: Lines) -> usize {
    who_compared(61, 17, instructions).unwrap()
}

pub fn part2(instructions: Lines) -> usize {
    process_instructions(instructions, ProcessAll).unwrap()
}

fn parse_file() -> Lines<'static> {
    include_str!("input/day10.txt").lines()
}

fn who_compared(value1: usize, value2: usize, instructions: Lines) -> Result<usize, &str> {
    process_instructions(instructions, CompareValues(value1, value2))
}

enum StopCondition {
    CompareValues(usize, usize),
    ProcessAll,
}

fn process_instructions(instructions: Lines, condition: StopCondition) -> Result<usize, &str> {
    let value_regex = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let compare_regex = Regex::new(r"^bot (\d+) gives low to (output|bot) (\d+) and high to (output|bot) (\d+)$").unwrap();
    let (values, comparisons): (Vec<_>, Vec<_>) = instructions
        .into_iter()
        .partition(|line| { line.starts_with("value") });
    let mut bot_values: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();
    for line in values {
        let capture = value_regex.captures(line).unwrap().unwrap();
        let value: usize = capture.index(1).parse().unwrap();
        let bot: usize = capture.index(2).parse().unwrap();
        give_chip_to_bot(&mut bot_values, value, bot);
    }
    let mut comparisons = VecDeque::from(comparisons.to_vec());
    while comparisons.len() > 0 {
        let line = comparisons.pop_front().unwrap();
        let capture = compare_regex.captures(line).unwrap().unwrap();
        let bot: usize = capture.index(1).parse().unwrap();
        let low_is_bot: bool = capture.index(2) == "bot";
        let low: usize = capture.index(3).parse().unwrap();
        let high_is_bot: bool = capture.index(4) == "bot";
        let high: usize = capture.index(5).parse().unwrap();
        let chips = bot_values.get(&bot);
        let chips_count = match chips {
            Some(vec) => vec.len(),
            None => 0
        };
        if chips_count != 2 {
            comparisons.push_back(line);
            continue;
        }
        let chips = chips.unwrap();
        let c1 = *chips.index(0);
        let c2 = *chips.index(1);
        if let CompareValues(value1, value2) = condition {
            if (c1 == value1 && c2 == value2) || (c1 == value2 && c2 == value1) {
                return Ok(bot);
            }
        }
        let (c1, c2) = if c1 > c2 { (c1, c2) } else { (c2, c1) };
        if high_is_bot {
            give_chip_to_bot(&mut bot_values, c1, high);
        } else {
            outputs.insert(high, c1);
        }
        if low_is_bot {
            give_chip_to_bot(&mut bot_values, c2, low);
        } else {
            outputs.insert(low, c2);
        }
        bot_values.insert(bot, Vec::new());
    }
    if let ProcessAll = condition {
        Ok(outputs.index(&0) * outputs.index(&1) * outputs.index(&2))
    } else {
        Err("Comparison not found")
    }
}

fn give_chip_to_bot(bot_values: &mut HashMap<usize, Vec<usize>>, chip: usize, bot: usize) {
    if !bot_values.contains_key(&bot) {
        bot_values.insert(bot, Vec::new());
    }
    bot_values.get_mut(&bot).unwrap().push(chip);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(56, part1(parse_file()));
    }

    #[test]
    fn part1_sample() {
        let instructions = include_str!("input/day10_sample.txt").lines();
        assert_eq!(2, who_compared(5, 2, instructions).unwrap());
    }

    #[test]
    fn part2_test() {
        assert_eq!(7847, part2(parse_file()));
    }
}
