use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::{Index};
use std::rc::Rc;
use std::str::Lines;

use fancy_regex::Regex;
use itertools::{Either, Itertools};

use crate::days::day11::Element::{Chip, Generator};

pub fn day11() {
    println!("=== Day 11 ===");
    println!("- Part 1");
    //let line = parse_file();
    //println!("{}", part1(line));
    println!("- Part 2");
    let line = parse_file();
    println!("{}", part2(line));
}

pub fn part1(instructions: Lines) -> usize {
    let floors = parse_instructions(instructions);
    shortest_path(floors)
}

fn shortest_path(floors: Floors) -> usize {
    let initial_state = State {
        elevator_pos: 0,
        floors,
        steps: 0,
    };

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(initial_state);
    while let Some(state) = heap.pop() {
        if visited.contains(&state) {
            continue;
        }
        if state.is_final() {
            return state.steps;
        }
        for next in state.next_states().into_iter() {
            heap.push(next)
        }
        visited.insert(state);
    }
    panic!("No end found");
}

fn parse_instructions(instructions: Lines) -> Floors {
    let chips_regex = Regex::new(r"(\S+)-compatible microchip").unwrap();
    let generators_regex = Regex::new(r"(\S+) generator").unwrap();
    let mut floors: [HashSet<Rc<Element>>; 4] = [HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
    for (index, line) in instructions.enumerate() {
        chips_regex.captures_iter(line).for_each(|capture| {
            floors[index].insert(Rc::new(Chip(String::from(capture.unwrap().index(1)))));
        });
        generators_regex.captures_iter(line).for_each(|capture| {
            floors[index].insert(Rc::new(Generator(String::from(capture.unwrap().index(1)))));
        });
    }
    floors
}

pub fn part2(instructions: Lines) -> usize {
    let mut floors = parse_instructions(instructions);
    floors[0].insert(Rc::new(Generator("elerium".to_string())));
    floors[0].insert(Rc::new(Generator("dilithium".to_string())));
    floors[0].insert(Rc::new(Chip("elerium".to_string())));
    floors[0].insert(Rc::new(Chip("dilithium".to_string())));
    shortest_path(floors)
}

fn parse_file() -> Lines<'static> {
    include_str!("input/day11.txt").lines()
}

#[derive(Debug, Eq, Clone)]
struct State {
    elevator_pos: usize,
    floors: Floors,
    steps: usize,
}

impl State {
    fn heuristic(&self) -> usize {
        self.floors.clone().into_iter().enumerate().map(|(i, v)| i * v.len()).sum()
    }

    fn is_final(&self) -> bool {
        self.floors[0..=2].into_iter().all(|v| v.is_empty())
    }

    fn next_states(&self) -> Vec<State> {
        let mut states = Vec::new();

        let combinations = self.compute_combinations();

        //Move up
        if self.elevator_pos < 3 {
            for inside_elevator in &combinations {
                let mut floors = self.floors.clone();

                let mut next_floor = HashSet::new();
                for element in floors[self.elevator_pos].clone().into_iter() {
                    if !inside_elevator.contains(&element) {
                        next_floor.insert(element);
                    }
                }
                floors[self.elevator_pos] = next_floor;
                for elem in inside_elevator.clone() {
                    floors[self.elevator_pos + 1].insert(elem);
                }
                let next_state = State {
                    elevator_pos: self.elevator_pos + 1,
                    floors,
                    steps: self.steps + 1,
                };
                if is_valid(next_state.floors[self.elevator_pos + 1].clone()) && is_valid(next_state.floors[self.elevator_pos].clone()) {
                    states.push(next_state);
                }
            }
        }
        //Move down
        if self.elevator_pos > 0 {
            for inside_elevator in &combinations {
                let mut floors = self.floors.clone();
                floors[self.elevator_pos] = floors[self.elevator_pos]
                    .clone()
                    .into_iter()
                    .filter(|e| !inside_elevator.contains(e))
                    .collect();
                for elem in inside_elevator.clone() {
                    floors[self.elevator_pos - 1].insert(elem);
                }
                let next_state = State {
                    elevator_pos: self.elevator_pos - 1,
                    floors,
                    steps: self.steps + 1,
                };
                if is_valid(next_state.floors[self.elevator_pos - 1].clone()) && is_valid(next_state.floors[self.elevator_pos].clone()) {
                    states.push(next_state);
                }
            }
        }
        states
    }

    fn compute_combinations(&self) -> Vec<HashSet<Rc<Element>>> {
        let current_floor = self.floors.index(self.elevator_pos);
        let mut combinations: Vec<HashSet<Rc<Element>>> = current_floor.into_iter().map(|e| HashSet::from([e.clone()])).collect();
        for (a, b) in current_floor.into_iter().tuple_combinations() {
            combinations.push(HashSet::from([a.clone(), b.clone()]));
        }
        combinations
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elevator_pos.hash(state);
        for floor in self.floors.clone() {
            floor.len().hash(state);
        }
    }
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        if self.elevator_pos != other.elevator_pos {
            return false;
        }
        for floor in 0..4 {
            let current_floor = self.floors.index(floor);
            let other_floor = other.floors.index(floor);
            if current_floor.len() != other_floor.len() {
                return false;
            }
            let current_chips = current_floor.into_iter().filter(|&e| { if let Chip(_) = e.as_ref() { true } else {false} }).count();
            let other_chips = other_floor.into_iter().filter(|&e| { if let Chip(_) = e.as_ref() { true } else {false} }).count();
            if current_chips != other_chips {
                return false;
            }
        }
        return true;
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
            .then_with(|| other.heuristic().cmp(&self.heuristic()))
    }
}

fn is_valid(floor: HashSet<Rc<Element>>) -> bool {
    let (chips, generators): (HashSet<_>, HashSet<_>) = floor
        .into_iter()
        .partition_map(|elem| match &*elem {
            Chip(ref name) => Either::Left(name.clone()),
            Generator(ref name) => Either::Right(name.clone())
        });
    generators.is_empty() || chips.into_iter().all(|name| generators.contains(&name))
}

type Floors = [HashSet<Rc<Element>>; 4];

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
enum Element {
    Chip(String),
    Generator(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test() {
        assert_eq!(State {
            elevator_pos: 0,
            floors: [
                HashSet::from([Rc::new(Generator("a".to_string())), Rc::new(Generator("b".to_string()))]),
                HashSet::new(),
                HashSet::new(),
                HashSet::new()],
            steps: 10,
        }, State {
            elevator_pos: 0,
            floors: [
                HashSet::from([Rc::new(Generator("b".to_string())), Rc::new(Generator("a".to_string()))]),
                HashSet::new(),
                HashSet::new(),
                HashSet::new()],
            steps: 3,
        });
        assert_ne!(State {
            elevator_pos: 1,
            floors: [
                HashSet::from([Rc::new(Generator("a".to_string())), Rc::new(Generator("b".to_string()))]),
                HashSet::new(),
                HashSet::new(),
                HashSet::new()],
            steps: 10,
        }, State {
            elevator_pos: 0,
            floors: [
                HashSet::from([Rc::new(Generator("b".to_string())), Rc::new(Generator("a".to_string()))]),
                HashSet::new(),
                HashSet::new(),
                HashSet::new()],
            steps: 10,
        });
    }

    #[test]
    fn part1_test() {
        assert_eq!(47, part1(parse_file()));
    }

    #[test]
    fn part1_sample() {
        let instructions = include_str!("input/day11_sample.txt").lines();
        assert_eq!(11, part1(instructions));
    }
}
