use std::cell::RefCell;
use std::collections::{VecDeque};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use itertools::Itertools;

pub fn day19() {
    let result = part1(3012210);
    println!("Part 1 = {}", result);
    let result = part2(3012210);
    println!("Part 2 = {}", result);
}

fn part1(elves: usize) -> usize {
    let mut gifts = vec![1usize; elves];
    let mut i = 0;
    loop {
        let mut next_index = if i < elves - 1 { i + 1 } else { 0 };
        while gifts[next_index] == 0 {
            next_index = if next_index < elves - 1 { next_index + 1 } else { 0 };
        }
        if gifts[i] != 0 {
            gifts[i] += gifts[next_index];
            if gifts[i] == elves {
                return i + 1;
            }
            gifts[next_index] = 0;
        }
        i = next_index;
    }
}

type Link = Option<Rc<RefCell<Elf>>>;

//#[derive(Debug)]
struct Elf {
    id: usize,
    next: Link,
}

impl Elf {
    fn set_next(&mut self, other: Rc<RefCell<Elf>>) {
        self.next = Some(other);
    }
}

impl Debug for Elf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Elf")
            .field("id", &self.id)
            .finish()
    }
}

fn part2(elves: usize) -> usize {
    let all_elves = (1..=elves)
        .map(|i| { Rc::new(RefCell::new(Elf { id: i, next: None })) })
        .collect_vec();

    for i in 0..elves {
        let current = all_elves[i].clone();
        let next = all_elves[(i + 1) % elves].clone();
        (*current).borrow_mut().set_next(next);
    }

    let mut current = all_elves[0].clone();

    let half = elves / 2;
    let mut before_opposite = current.clone();
    for _ in 1..half {
        before_opposite = (*before_opposite.clone()).borrow().next.as_ref().unwrap().clone();
    }

    for round in 0..=(elves - 2) {
        {
            let mut borrowed = (*before_opposite).borrow_mut();
            let after_opposite = (*borrowed.next.as_ref().unwrap()).borrow().next.as_ref().unwrap().clone();
            borrowed.next = Some(after_opposite.clone());
        }

        current = (*current.clone()).borrow().next.as_ref().unwrap().clone();

        if (elves - round) % 2 == 1 {
            before_opposite = (*before_opposite.clone()).borrow().next.as_ref().unwrap().clone();
        }
    }

    (*current.clone()).borrow_mut().id
}

#[allow(unused)]
fn part2_old(elves: usize) -> usize {
    let mut gifts = VecDeque::new();
    for i in 1..=elves {
        gifts.push_back(i);
    }
    let mut i = 0;
    for round in 0..elves - 1 {
        let half = (elves - round) / 2;
        let opposite = (i + half) % (elves - round);
        gifts.remove(opposite);
        i = (i + 1) % (elves - round);
    }
    gifts[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(3, part1(5));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(2, part2(5));
    }
}
