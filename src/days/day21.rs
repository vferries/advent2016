use itertools::Itertools;
use crate::days::day21::Instruction::{MovePositions, ReversePositions, RotateLeft, RotateLetter, RotateRight, SwapLetters, SwapPositions};

pub fn day21() {
    let instructions = parse_file(include_str!("input/day21.txt"));
    let input = "abcdefgh";
    let result = part1(input, &instructions);
    println!("Part 1 = {}", result);
    let instructions = parse_file(include_str!("input/day21.txt"));
    let input = "fbgdceah";
    let result = part2(input, &instructions);
    println!("Part 2 = {}", result);
}

enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    ReversePositions(usize, usize),
    MovePositions(usize, usize),
}

fn parse_file(file_content: &str) -> Vec<Instruction> {
    file_content.lines().map(|line| {
        if line.starts_with("swap position") {
            let x = line.chars().nth(14).unwrap().to_digit(10).unwrap() as usize;
            let y = line.chars().last().unwrap().to_digit(10).unwrap() as usize;
            SwapPositions(x, y)
        } else if line.starts_with("swap letter") {
            let x = line.chars().nth(12).unwrap();
            let y = line.chars().last().unwrap();
            SwapLetters(x, y)
        } else if line.starts_with("rotate left") {
            let x = line.chars().nth(12).unwrap().to_digit(10).unwrap() as usize;
            RotateLeft(x)
        } else if line.starts_with("rotate right") {
            let x = line.chars().nth(13).unwrap().to_digit(10).unwrap() as usize;
            RotateRight(x)
        } else if line.starts_with("rotate based") {
            let c = line.chars().last().unwrap();
            RotateLetter(c)
        } else if line.starts_with("reverse positions") {
            let x = line.chars().nth(18).unwrap().to_digit(10).unwrap() as usize;
            let y = line.chars().last().unwrap().to_digit(10).unwrap() as usize;
            ReversePositions(x, y)
        } else if line.starts_with("move position") {
            let x = line.chars().nth(14).unwrap().to_digit(10).unwrap() as usize;
            let y = line.chars().last().unwrap().to_digit(10).unwrap() as usize;
            MovePositions(x, y)
        } else {
            panic!("Unknown instruction")
        }
    }
    ).collect()
}

fn part1(input: &str, instructions: &Vec<Instruction>) -> String {
    let mut chars = input.chars().collect_vec();
    for instruction in instructions {
        match *instruction {
            SwapPositions(x , y) => {
                chars.swap(x, y);
            }
            SwapLetters(x, y) => {
                let i = chars.iter().position(|c| c == &x).unwrap();
                let j = chars.iter().position(|c| c == &y).unwrap();
                chars.swap(i, j);
            }
            RotateLeft(x) => {
                chars.rotate_left(x)
            }
            RotateRight(x) => {
                chars.rotate_right(x)
            }
            RotateLetter(x) => {
                let i = chars.iter().position(|c| c == &x).unwrap();
                chars.rotate_right(i + 1);
                if i >= 4 {
                    chars.rotate_right(1);
                }
            }
            ReversePositions(mut x, mut y) => {
                while x < y {
                    chars.swap(x, y);
                    x += 1;
                    y -= 1;
                }
            }
            MovePositions(x, y) => {
                let c = chars.remove(x);
                chars.insert(y, c);
            }
        }
    }
    return chars.iter().cloned().collect();
}

fn part2(input: &str, instructions: &Vec<Instruction>) -> String {
    let chars = input.chars().collect_vec();
    let permutations = chars.iter().permutations(chars.len());
    for permutation in permutations {
        let p: String = permutation.iter().cloned().collect();
        if part1(p.as_str(), instructions) == "fbgdceah" {
            return p;
        }
    }
    panic!("Not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let instructions = parse_file(include_str!("input/day21_sample.txt"));
        assert_eq!("decab", part1("abcde", &instructions));
    }

    #[test]
    fn part1_input() {
        let instructions = parse_file(include_str!("input/day21.txt"));
        assert_eq!("bgfacdeh", part1("abcdefgh", &instructions));
    }

    #[test]
    fn part2_input() {
        let instructions = parse_file(include_str!("input/day21.txt"));
        assert_eq!("bdgheacf", part2("fbgdceah", &instructions));
    }
}
