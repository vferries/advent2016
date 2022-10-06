use std::collections::HashMap;
use std::str::Lines;

pub fn day6() {
    println!("=== Day 6 ===");
    println!("- Part 1");
    let mut lines = parse_file();
    println!("{}", part1(&mut lines));
    println!("- Part 2");
    let mut lines = parse_file();
    println!("{}", part2(&mut lines));
}

pub fn part1(messages: &mut Lines) -> String {
    let mut result = String::new();
    let mut count: HashMap<(char, usize), usize> = HashMap::new();
    let mut width = 0;
    for message in messages.into_iter() {
        width = message.len();
        for i in 0..width {
            let char = message.chars().nth(i).unwrap();
            let new_val = count.get(&(char, i)).unwrap_or(&0) + 1;
            count.insert((char, i), new_val);
        }
    }
    for i in 0..width {
        let ((most_represented, _), _) = count.iter()
            .filter(|((_, index), _)| { *index == i })
            .max_by(|a, b| { a.1.cmp(b.1) }).unwrap();
        result.push(*most_represented);
    }
    result
}

pub fn part2(messages: &mut Lines) -> String {
    let mut result = String::new();
    let mut count: HashMap<(char, usize), usize> = HashMap::new();
    let mut width = 0;
    for message in messages.into_iter() {
        width = message.len();
        for i in 0..width {
            let char = message.chars().nth(i).unwrap();
            let new_val = count.get(&(char, i)).unwrap_or(&0) + 1;
            count.insert((char, i), new_val);
        }
    }
    for i in 0..width {
        let ((most_represented, _), _) = count.iter()
            .filter(|((_, index), _)| { *index == i })
            .min_by(|a, b| { a.1.cmp(b.1) }).unwrap();
        result.push(*most_represented);
    }
    result
}

fn parse_file() -> Lines<'static> {
    include_str!("input/day6.txt").lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let mut messages = include_str!("input/day6_sample.txt").lines();
        assert_eq!("easter", part1(&mut messages));
    }

    #[test]
    fn part1_real() {
        let mut messages = include_str!("input/day6.txt").lines();
        assert_eq!("zcreqgiv", part1(&mut messages));
    }

    #[test]
    fn part2_sample() {
        let mut messages = include_str!("input/day6_sample.txt").lines();
        assert_eq!("advent", part2(&mut messages));
    }

    #[test]
    fn part2_real() {
        let mut messages = include_str!("input/day6.txt").lines();
        assert_eq!("pljvorrk", part2(&mut messages));
    }
}
