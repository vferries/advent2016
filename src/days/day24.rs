use std::collections::{HashMap, HashSet, VecDeque};
use std::str::Lines;

use itertools::Itertools;

pub fn day24() {
    let lines = parse_file();
    let result = part1(lines);
    println!("Part 1 = {}", result);
    let lines = parse_file();
    let result = part2(lines);
    println!("Part 2 = {}", result);
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        vec!(
            Position { x: self.x - 1, y: self.y },
            Position { x: self.x + 1, y: self.y },
            Position { x: self.x, y: self.y - 1 },
            Position { x: self.x, y: self.y + 1 })
    }
}

fn part1(lines: Lines) -> usize {
    let mut destinations: HashSet<Position> = HashSet::new();
    let mut numbers: HashMap<u8, Position> = HashMap::new();
    let mut init_pos: Position = Position { x: 0, y: 0 };
    let mut shortest_paths: HashMap<(u8, u8), usize> = HashMap::new();

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Position { x, y };
            match char {
                '#' => {}// skip
                '.' => {
                    destinations.insert(pos);
                }
                _ => {
                    destinations.insert(pos);
                    numbers.insert(char.to_digit(10).unwrap() as u8, pos);
                }
            }
        }
    }
    init_pos = *numbers.get(&0).unwrap();

    // Shortest paths
    for (n, pos) in numbers.clone() {
        let mut to_visit: VecDeque<(Position, usize)> = VecDeque::new();
        to_visit.push_back((pos, 0));
        let mut visited: HashSet<Position> = HashSet::new();

        while !to_visit.is_empty() {
            let (next_pos, steps) = to_visit.pop_front().unwrap();
            if visited.contains(&next_pos) {
                continue;
            }
            visited.insert(next_pos);
            if numbers.values().contains(&next_pos) {
                // get number
                let m = *numbers.iter().find(|(k, v)| **v == next_pos).unwrap().0;
                // add to shortest_paths
                shortest_paths.insert((n, m), steps);
            }

            // add neighbors
            for neighbor in next_pos.neighbors().iter().filter(|p| destinations.contains(p)) {
                to_visit.push_back((*neighbor, steps + 1));
            }
        }
    }

    let permutations = numbers.keys().permutations(numbers.len())
        .filter(|p| *p.get(0).unwrap().clone() == 0);
    permutations.map(|p| {
        p.windows(2)
            .map(|w| { shortest_paths.get(&(*w[0], *w[1])).unwrap() }).sum()
    }).min().unwrap()
}

fn part2(lines: Lines) -> usize {
    let mut destinations: HashSet<Position> = HashSet::new();
    let mut numbers: HashMap<u8, Position> = HashMap::new();
    let mut init_pos: Position = Position { x: 0, y: 0 };
    let mut shortest_paths: HashMap<(u8, u8), usize> = HashMap::new();

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Position { x, y };
            match char {
                '#' => {}// skip
                '.' => {
                    destinations.insert(pos);
                }
                _ => {
                    destinations.insert(pos);
                    numbers.insert(char.to_digit(10).unwrap() as u8, pos);
                }
            }
        }
    }
    init_pos = *numbers.get(&0).unwrap();

    // Shortest paths
    for (n, pos) in numbers.clone() {
        let mut to_visit: VecDeque<(Position, usize)> = VecDeque::new();
        to_visit.push_back((pos, 0));
        let mut visited: HashSet<Position> = HashSet::new();

        while !to_visit.is_empty() {
            let (next_pos, steps) = to_visit.pop_front().unwrap();
            if visited.contains(&next_pos) {
                continue;
            }
            visited.insert(next_pos);
            if numbers.values().contains(&next_pos) {
                // get number
                let m = *numbers.iter().find(|(k, v)| **v == next_pos).unwrap().0;
                // add to shortest_paths
                shortest_paths.insert((n, m), steps);
            }

            // add neighbors
            for neighbor in next_pos.neighbors().iter().filter(|p| destinations.contains(p)) {
                to_visit.push_back((*neighbor, steps + 1));
            }
        }
    }

    let permutations = numbers.keys().permutations(numbers.len())
        .filter(|p| *p.get(0).unwrap().clone() == 0);
    permutations.map(|p| {
        shortest_paths.get(&(**p.last().unwrap(), 0)).unwrap() +
        p.windows(2)
            .map(|w| { shortest_paths.get(&(*w[0], *w[1])).unwrap() }).sum::<usize>()
    }).min().unwrap()
}


fn parse_file() -> Lines<'static> {
    include_str!("input/day24.txt").lines()
}

fn char_at(str: &str, index: usize) -> char {
    str.chars().nth(index).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let sample = include_str!("input/day24_sample.txt").lines();
        assert_eq!(14, part1(sample));
    }

    #[test]
    fn part1_input() {
        assert_eq!(464, part1(parse_file()));
    }
    #[test]
    fn part2_sample() {
        let sample = include_str!("input/day24_sample.txt").lines();
        assert_eq!(20, part2(sample));
    }

    #[test]
    fn part2_input() {
        assert_eq!(652, part2(parse_file()));
    }
}
