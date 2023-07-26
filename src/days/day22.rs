extern crate pathfinding;

use std::collections::{BTreeMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use pathfinding::prelude::astar;

pub fn day22() {
    let nodes = parse_file(include_str!("input/day22.txt"));
    let result = part2(nodes);
    println!("Part 2 = {}", result);
}

#[derive(Debug, Hash, Clone)]
struct Node {
    x: usize,
    y: usize,
    used: usize,
    available: usize,
}

fn parse_file(file_content: &str) -> Vec<Node> {
    file_content.lines().skip(2).map(|line| {
        let parts = line.split_whitespace().collect_vec();
        let used = parts[2].trim_end_matches("T").parse().unwrap();
        let available = parts[3].trim_end_matches("T").parse().unwrap();
        let position = parts[0].split("-").collect_vec();
        let x = position[1].trim_start_matches("x").parse().unwrap();
        let y = position[2].trim_start_matches("y").parse().unwrap();
        Node {
            x,
            y,
            used,
            available,
        }
    }
    ).collect()
}

fn part2(nodes: Vec<Node>) -> usize {
    let mut nodes_map = BTreeMap::new();
    for node in nodes {
        nodes_map.insert((node.x, node.y), node);
    }
    let max_x = nodes_map.keys().max_by(|(x1, _), (x2, _)| { x1.cmp(x2) }).unwrap().0;

    let start = State {
        nodes_map: nodes_map.clone(),
        goal: (max_x, 0),
        empty: *nodes_map.iter().find(|(_, v)| v.used == 0).unwrap().0,
    };

    let option = astar(&start, |s| s.successors(), |s| h(s), |s| s.goal == (0, 0));
    option.expect("No result found").1
}

fn h(state: &State) -> usize {
    state.goal.0 + state.goal.1
}

fn neighbors<'a>(node: &'a Pos, nodes_map: &'a BTreeMap<Pos, Node>) -> Vec<&'a Node> {
    let mut neighbors = Vec::new();
    if node.0 > 0 && nodes_map.contains_key(&(node.0 - 1, node.1)) {
        neighbors.push(nodes_map.get(&(node.0 - 1, node.1)).unwrap());
    }
    if node.1 > 0 && nodes_map.contains_key(&(node.0, node.1 - 1)) {
        neighbors.push(nodes_map.get(&(node.0, node.1 - 1)).unwrap());
    }
    if nodes_map.contains_key(&(node.0, node.1 + 1)) {
        neighbors.push(nodes_map.get(&(node.0, node.1 + 1)).unwrap());
    }
    if nodes_map.contains_key(&(node.0 + 1, node.1)) {
        neighbors.push(nodes_map.get(&(node.0 + 1, node.1)).unwrap());
    }
    neighbors
}

type Pos = (usize, usize);

#[derive(Clone, Debug)]
struct State {
    nodes_map: BTreeMap<Pos, Node>,
    goal: Pos,
    empty: Pos,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "State [ Goal = ({}, {}), Empty = ({}, {}) ]", self.goal.0, self.goal.1, self.empty.0, self.empty.1)
    }
}

impl State {
    fn successors(&self) -> Vec<(Self, usize)> {
        let mut result = Vec::new();
        let map = &self.nodes_map;

        let mut to_visit = vec![(self.empty, 0)];
        let mut visited = HashSet::new();
        while let Some((pos, steps)) = to_visit.pop() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            if (self.goal.0.abs_diff(pos.0) + self.goal.1.abs_diff(pos.1)) == 1 {
                let mut new_map = map.clone();
                let new_empty = Node {
                    x: self.empty.0,
                    y: self.empty.1,
                    used: 60,
                    available: 20,
                };
                new_map.insert(self.empty, new_empty);

                let new_goal = Node {
                    x: self.goal.0,
                    y: self.goal.1,
                    used: 0,
                    available: 80,
                };
                new_map.insert(self.goal, new_goal);

                let new_pos = Node {
                    x: pos.0,
                    y: pos.1,
                    used: 60,
                    available: 20,
                };
                new_map.insert(pos, new_pos);

                let next = State {
                    goal: pos,
                    empty: self.goal,
                    nodes_map: new_map,
                };

                result.push((next, steps + 1));
            }
            for neighbor in neighbors(&pos, map) {
                if (neighbor.x, neighbor.y) != self.goal {
                    to_visit.insert(0, ((neighbor.x, neighbor.y), steps + 1));
                }
            }
        }
        println!("New shortest path for {}", self);
        for (state, cost) in result.clone().into_iter() {
            println!("Shortest path {} is {}", state, cost);
        }
        result
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.goal.hash(state);
        self.empty.hash(state);
    }
}

impl Eq for State {}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.goal == other.goal && self.empty == other.empty
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y) == (other.x, other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_sample() {
        let nodes = parse_file(include_str!("input/day22_sample.txt"));
        assert_eq!(7, part2(nodes));
    }

    // Should be > 132 && < 8874
}
