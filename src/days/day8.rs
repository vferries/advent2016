use std::ops::Index;
use std::str::Lines;

use fancy_regex::Regex;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

pub fn day8() {
    println!("=== Day 8 ===");
    println!("- Part 1");
    let mut lines = parse_file();
    println!("{}", part1(&mut lines));
    println!("- Part 2");
    let mut lines = parse_file();
    part2(&mut lines);
}

pub fn part1(lines: &mut Lines) -> usize {
    let mut grid = [[false; WIDTH]; HEIGHT];
    process_commands(lines, &mut grid);
    count_lit_pixels(&mut grid)
}

fn process_commands(lines: &mut Lines, mut grid: &mut [[bool; WIDTH]; HEIGHT]) {
    let rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let rotate_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let rotate_col = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    for line in lines {
        if rect.is_match(line).unwrap() {
            let capture = rect.captures(line).unwrap().unwrap();
            let x: usize = capture.index(1).parse().unwrap();
            let y: usize = capture.index(2).parse().unwrap();
            fill_rect(&mut grid, x, y);
        } else if rotate_row.is_match(line).unwrap() {
            let capture = rotate_row.captures(line).unwrap().unwrap();
            let row: usize = capture.index(1).parse().unwrap();
            let offset: usize = capture.index(2).parse().unwrap();
            rotate_row_fn(&mut grid, row, offset);
        } else {
            let capture = rotate_col.captures(line).unwrap().unwrap();
            let col: usize = capture.index(1).parse().unwrap();
            let offset: usize = capture.index(2).parse().unwrap();
            rotate_col_fn(&mut grid, col, offset);
        }
    }
}

fn count_lit_pixels(grid: &mut [[bool; WIDTH]; HEIGHT]) -> usize {
    let mut count: usize = 0;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if grid[row][col] { count += 1; }
        }
    }
    count
}

fn fill_rect(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    for i in 0..x {
        for j in 0..y {
            grid[j][i] = true;
        }
    }
}

fn rotate_row_fn(grid: &mut [[bool; WIDTH]; HEIGHT], row: usize, offset: usize) {
    let old = grid[row].clone();
    for i in 0..WIDTH {
        let new_col = (i + offset) % WIDTH;
        grid[row][new_col] = old[i];
    }
}

fn rotate_col_fn(grid: &mut [[bool; WIDTH]; HEIGHT], col: usize, offset: usize) {
    let mut old = [false; HEIGHT];
    for i in 0..HEIGHT {
        old[i] = grid[i][col];
    }
    for i in 0..HEIGHT {
        let new_row = (i + offset) % HEIGHT;
        grid[new_row][col] = old[i];
    }
}

pub fn part2(lines: &mut Lines) {
    let mut grid = [[false; WIDTH]; HEIGHT];
    process_commands(lines, &mut grid);
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            print!("{}", if grid[row][col] { "#"} else {"."});
        }
        println!();
    }
}

fn parse_file() -> Lines<'static> {
    include_str!("input/day8.txt").lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut lines = include_str!("input/day8.txt").lines();
        assert_eq!(119, part1(&mut lines));
    }

    #[test]
    fn part1_sample() {
        let mut lines = "rect 5x5\nrotate row y=2 by 4".lines();
        assert_eq!(25, part1(&mut lines));
    }
}
