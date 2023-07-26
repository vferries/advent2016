use std::collections::HashMap;
use std::ops::Index;

use itertools::{Either, Itertools};
use itertools::Either::{Left, Right};

use crate::days::day23::Instruction::{Copy, Dec, Inc, JumpIfNonZero, Toggle};

pub fn day23() {
    let instructions = parse_file(include_str!("input/day23.txt"));
    let result = part1(instructions);
    println!("Part 1 = {}", result);

    let instructions = parse_file(include_str!("input/day23.txt"));
    let result = part2(instructions);
    println!("Part 2 = {}", result);
}

fn part1(instructions: Vec<Instruction>) -> i32 {
    let mut registers = HashMap::new();
    registers.insert('a', 7);
    execute(instructions, registers, 0)
}

fn factorial(num: i32) -> i32 {
    (1..=num).product()
}

fn part2_old(mut instructions: Vec<Instruction>) -> i32 {
    let mut registers = HashMap::new();
    registers.insert('a', 12);
    execute(instructions, registers, 0)
}

fn part2(mut instructions: Vec<Instruction>) -> i32 {
    let mut registers = HashMap::new();
    for i in (20..instructions.len()).step_by(2) {
        let mod_instruction = instructions.index(i);
        instructions[i] = match mod_instruction {
            Inc(r) => Dec(*r),
            Dec(r) => Inc(*r),
            Toggle(r) => Inc(*r),
            Copy(either, r) => JumpIfNonZero(*either, *r),
            JumpIfNonZero(either, r) => Copy(*either, *r)
        };
    }
    registers.insert('a', factorial(12));
    registers.insert('b', 1);
    execute(instructions, registers, 19)
}

fn execute(mut instructions: Vec<Instruction>, mut registers: HashMap<char, i32>, mut index: i32) -> i32 {
    while index >= 0 && index < instructions.len() as i32 {
        match instructions.index(index as usize) {
            Copy(either, r) => {
                index += 1;
                let value = match either {
                    Left(r2) => registers[r2],
                    Right(v) => *v
                };
                if let Left(c) = r {
                    registers.insert(*c, value);
                }
            }
            Inc(r) => {
                index += 1;
                let val = registers.entry(*r).or_insert(0);
                *val += 1
            }
            Toggle(r) => {
                let val = registers.entry(*r).or_insert(0);
                let mod_index = *val + index;
                if mod_index >= 0 && mod_index < instructions.len() as i32 {
                    let mod_index = mod_index as usize;
                    let mod_instruction = instructions.index(mod_index);
                    instructions[mod_index] = match mod_instruction {
                        Inc(r) => Dec(*r),
                        Dec(r) => Inc(*r),
                        Toggle(r) => Inc(*r),
                        Copy(either, r) => JumpIfNonZero(*either, *r),
                        JumpIfNonZero(either, r) => Copy(*either, *r)
                    }
                }
                index += 1;
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
                index += if value == 0 { 1 } else {
                    match offset {
                        Left(r2) => *registers.get(r2).unwrap_or(&0),
                        Right(offset) => *offset
                    }
                };
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
                "tgl" => Toggle(first),
                "cpy" => {
                    let param2 = *parts.index(2);
                    let either = if first.is_alphabetic() {
                        Left(first)
                    } else { Right(param1.parse().unwrap()) };
                    Copy(either, Left(char_at(param2, 0)))
                }
                "jnz" => {
                    let param2 = *parts.index(2);
                    let either = if first.is_alphabetic() {
                        Left(first)
                    } else { Right(param1.parse().unwrap()) };
                    let offset = if char_at(param2, 0).is_alphabetic() {
                        Left(char_at(param2, 0))
                    } else { Right(param2.parse().unwrap()) };
                    JumpIfNonZero(either, offset)
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
    Inc(char),
    Dec(char),
    Toggle(char),
    Copy(Either<char, i32>, Either<char, i32>),
    JumpIfNonZero(Either<char, i32>, Either<char, i32>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let sample = include_str!("input/day23_sample.txt");
        assert_eq!(3, part1(parse_file(sample)));
    }

    #[test]
    fn part1_input() {
        let input = include_str!("input/day23.txt");
        assert_eq!(11662, part1(parse_file(input)));
    }

    #[test]
    fn part2_input() {
        let input = include_str!("input/day23.txt");
        assert_eq!(479008222, part2(parse_file(input)));
    }
}
