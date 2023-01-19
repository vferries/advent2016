use std::collections::VecDeque;

use itertools::Itertools;

pub fn day17() {
    let input = "veumntbg";
    let result = part1(input);
    println!("Part 1 = {}", result);
    let result = part2(input);
    println!("Part 2 = {}", result);
}

trait State {
    fn next_states(&self) -> Vec<Self> where Self: Sized;
}

#[derive(Debug)]
struct MazeState {
    position: (u8, u8),
    path: String,
}

impl State for MazeState {
    fn next_states(&self) -> Vec<MazeState> where Self: Sized {
        let digest = md5::compute(self.path.clone());
        let md5 = &format!("{:x}", digest)[0..4];
        let directions = "UDLR";
        (0..=3).filter(|i| {
            let c = md5.chars().nth(*i).unwrap();
            ('b'..='f').contains(&c)
        })
            .map(|i| MazeState {
                position: next_pos(self.position, i),
                path: format!("{}{}", self.path, directions.chars().nth(i).unwrap()),
            })
            .filter(|s| (1..=4).contains(&s.position.0) && (1..=4).contains(&s.position.1))
            .collect_vec()
    }
}

fn next_pos(pos: (u8, u8), index: usize) -> (u8, u8) {
    match index {
        0 => (pos.0, pos.1 - 1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 - 1, pos.1),
        3 => (pos.0 + 1, pos.1),
        _ => panic!("404 - Index not found")
    }
}

fn part1(input: &str) -> String {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(MazeState { position: (1, 1), path: String::from(input) });
    while !to_visit.is_empty() {
        let state = to_visit.pop_front().unwrap();
        if state.position == (4, 4) {
            return String::from(&state.path[input.len()..state.path.len()]);
        }
        for next in state.next_states() {
            to_visit.push_back(next);
        }
    }
    panic!("404 - Path not found !");
}

fn part2(input: &str) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(MazeState { position: (1, 1), path: String::from(input) });
    let mut best = 0;
    while !to_visit.is_empty() {
        let state = to_visit.pop_front().unwrap();
        if state.position == (4, 4) {
            best = state.path.len() - input.len();
        } else {
            for next in state.next_states() {
                to_visit.push_back(next);
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!("DDRRRD", part1("ihgpwlah"));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!("DDUDRLRRUDRD", part1("kglvqrro"));
    }

    #[test]
    fn part1_sample3() {
        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", part1("ulqzkmiv"));
    }

    #[test]
    fn part1_input() {
        assert_eq!("DDRRULRDRD", part1("veumntbg"));
    }

    #[test]
    fn part2_sample1() {
        assert_eq!(370, part2("ihgpwlah"));
    }

    #[test]
    fn part2_sample2() {
        assert_eq!(492, part2("kglvqrro"));
    }

    #[test]
    fn part2_sample3() {
        assert_eq!(830, part2("ulqzkmiv"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(536, part2("veumntbg"));
    }
}
