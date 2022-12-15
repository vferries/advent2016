use std::collections::HashMap;
use std::ops::Index;

use itertools::{Either, Itertools};
use itertools::Either::{Left, Right};

use crate::days::day12::Instruction::{Copy, Dec, Inc, JumpIfNonZero};

pub fn day12() {
    let instructions = parse_file(include_str!("input/day12.txt"));
    let result = part1(instructions);
    println!("Part 1 = {}", result);

    let instructions = parse_file(include_str!("input/day12.txt"));
    let result = part2(instructions);
    println!("Part 2 = {}", result);
}

fn part2(instructions: Vec<Instruction>) -> i32 {
    let mut registers = HashMap::new();
    registers.insert('c', 1);
    execute(instructions, registers)
}

fn part1(instructions: Vec<Instruction>) -> i32 {
    let mut registers = HashMap::new();
    execute(instructions, registers)
}

fn execute(instructions: Vec<Instruction>, mut registers: HashMap<char, i32>) -> i32 {
    let mut index: i32 = 0;
    while index >= 0 && index < instructions.len() as i32 {
        match instructions.index(index as usize) {
            Copy(either, r) => {
                index += 1;
                let value = match either {
                    Left(r2) => registers[r2],
                    Right(v) => *v
                };
                registers.insert(*r, value);
            }
            Inc(r) => {
                index += 1;
                let val = registers.entry(*r).or_insert(0);
                *val += 1
            }
            Dec(r) => {
                index += 1;
                let val = registers.entry(*r).or_insert(0);
                *val -= 1
            }
            JumpIfNonZero(either, offset) => {
                let value = match either {
                    Left(r2) => *registers.get(r2).unwrap_or(&0),
                    Right(v) => *v
                };
                index += if value == 0 { 1 } else { *offset }
            }
        }
    }
    registers[&'a']
}

fn parse_file(file_content: &str) -> Vec<Instruction> {
    file_content.lines().map(|line| line.split(" ").collect_vec())
        .map(|parts| {
            let command = *parts.index(0);
            let param1 = *parts.index(1);
            let first = char_at(param1, 0);
            match command {
                "dec" => Dec(first),
                "inc" => Inc(first),
                "cpy" => {
                    let param2 = *parts.index(2);
                    let either = if first.is_alphabetic() {
                        Left(first)
                    } else { Right(param1.parse().unwrap()) };
                    Copy(either, char_at(param2, 0))
                }
                "jnz" => {
                    let param2 = *parts.index(2);
                    let either = if first.is_alphabetic() {
                        Left(first)
                    } else { Right(param1.parse().unwrap()) };
                    JumpIfNonZero(either, param2.parse().unwrap())
                }
                _ => panic!("Command {} unknown", command)
            }
        }).collect_vec()
}

fn char_at(str: &str, index: usize) -> char {
    str.chars().nth(index).unwrap()
}

#[derive(Debug)]
enum Instruction {
    Copy(Either<char, i32>, char),
    Inc(char),
    Dec(char),
    JumpIfNonZero(Either<char, i32>, i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let sample = include_str!("input/day12_sample.txt");
        assert_eq!(42, part1(parse_file(sample)));
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input/day12.txt");
        assert_eq!(318077, part1(parse_file(input)));
    }
}
