use std::collections::HashSet;

pub fn day13() {
    let input = 1350;
    let dest = Pos(31, 39);
    let result = part1(input, dest);
    println!("Part 1 = {}", result);

    println!("Part 2 = {}", 0);
}

fn part1(input: usize, dest: Pos) -> i32 {
    let init_pos = Pos(1, 1);
    let mut to_visit = vec!((init_pos, 0));
    let mut visited = HashSet::new();
    while !to_visit.is_empty() {
        let (pos, steps) = to_visit.remove(0);
        if pos == dest {
            return steps;
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        let neighbors = pos.neighbors();
        for neighbor in neighbors {
            if is_empty(input, pos) {
                to_visit.push((neighbor, steps + 1));
            }
        }
    }
    panic!("Path not found")
}

#[allow(unused)]
fn part2(input: usize) -> usize {
    let init_pos = Pos(1, 1);
    let mut to_visit = vec!((init_pos, 0));
    let mut visited = HashSet::new();
    while !to_visit.is_empty() {
        let (pos, steps) = to_visit.remove(0);
        if steps > 50 || visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        let neighbors = pos.neighbors();
        for neighbor in neighbors {
            if is_empty(input, neighbor) {
                to_visit.push((neighbor, steps + 1));
            }
        }
    }
    println!("{:?}", visited);
    return visited.len();
}

fn is_empty(input: usize, pos: Pos) -> bool {
    let Pos(x, y) = pos;
    let v:usize = x*x + 3*x + 2*x*y + y + y*y + input;
    v.count_ones() % 2 == 0
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Pos(usize, usize);

impl Pos {
    pub(crate) fn neighbors(&self) -> Vec<Pos> {
        let mut neighbors = vec!(Pos(self.0 + 1, self.1), Pos(self.0, self.1 + 1));
        if self.0 > 0 {
            neighbors.push(Pos(self.0 - 1, self.1));
        }
        if self.1 > 0 {
            neighbors.push( Pos(self.0, self.1 - 1));
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(11, part1(10, Pos(7, 4)));
    }

    #[test]
    fn part1_input() {
        let input = 1350;
        let dest = Pos(31, 39);
        assert_eq!(92, part1(input, dest));
    }

    #[test]
    fn part2_input() {
        let input = 1350;
        assert_eq!(124, part2(input));
    }
}
