use std::collections::HashSet;

pub fn day1() {
    let instructions = parse_file();

    let (x, y) = compute_destination(&instructions);

    println!("Part 1 = {}", x.abs() + y.abs());

    let (x, y) = first_repetition(&instructions);

    println!("Part 2 = {}", x.abs() + y.abs());
}

fn parse_file() -> Vec<Instruction> {
    let content = include_str!("input/day1.txt");
    let content = content.split(", ");
    let instructions: Vec<Instruction> = content.map(|instruction| {
        let direction = &instruction.chars().nth(0).unwrap();
        let steps = &instruction[1..];
        let steps: i32 = steps.parse().unwrap();
        Instruction::from(*direction, steps)
    }).collect();
    instructions
}

fn compute_destination(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut direction = Direction::NORTH;
    let mut position = (0, 0);
    for instruction in instructions {
        direction = direction.rotate(instruction.direction);
        position = direction.walk(position, instruction.steps);
    }
    position
}

fn first_repetition(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut direction = Direction::NORTH;
    let mut position = (0, 0);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    seen.insert(position);
    for instruction in instructions {
        direction = direction.rotate(instruction.direction);
        for _ in 0..instruction.steps {
            position = direction.walk(position, 1);
            if seen.contains(&position) {
                return position;
            } else {
                seen.insert(position);
            }
        }
    }
    panic!("No repetition found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let dest = compute_destination(&parse_file());
        assert_eq!((-172, 127), dest);
        assert_eq!(299, dest.0.abs() + dest.1.abs());
    }

    #[test]
    fn single_step() {
        let dest = compute_destination(&vec![Instruction::from('R', 1)]);
        assert_eq!((1, 0), dest);
    }

    #[test]
    fn two_steps() {
        let dest = compute_destination(&vec![Instruction::from('L', 3), Instruction::from('L', 4)]);
        assert_eq!((-3, 4), dest);
    }

    #[test]
    fn part2() {
        let dest = first_repetition(&parse_file());
        assert_eq!((-176, 5), dest);
        assert_eq!(181, dest.0.abs() + dest.1.abs());
    }

    #[test]
    fn first_rep() {
        let dest = first_repetition(&vec![
            Instruction::from('R', 8),
            Instruction::from('R', 4),
            Instruction::from('R', 4),
            Instruction::from('R', 8),
        ]);
        assert_eq!((4, 0), dest);
    }
}

#[derive(Debug)]
struct Instruction {
    direction: char,
    steps: i32,
}

impl Instruction {
    fn from(direction: char, steps: i32) -> Self {
        Instruction {
            direction,
            steps,
        }
    }
}

enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub(crate) fn walk(&self, (x, y): (i32, i32), steps: i32) -> (i32, i32) {
        match self {
            Direction::NORTH => { (x, y - steps) }
            Direction::EAST => { (x + steps, y) }
            Direction::SOUTH => { (x, y + steps) }
            Direction::WEST => { (x - steps, y) }
        }
    }
}

impl Direction {
    fn rotate(&self, dir: char) -> Direction {
        match self {
            Direction::NORTH => if dir == 'L' { Direction::WEST } else { Direction::EAST }
            Direction::EAST => if dir == 'L' { Direction::NORTH } else { Direction::SOUTH }
            Direction::SOUTH => if dir == 'L' { Direction::EAST } else { Direction::WEST }
            Direction::WEST => if dir == 'L' { Direction::SOUTH } else { Direction::NORTH }
        }
    }
}
