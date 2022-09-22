use std::str::Lines;

pub fn day3() {
    println!("=== Day 3 ===");
    println!("- Part 1");
    let lines = parse_file();
    println!("{}", valid_count(lines));
}

fn valid_count(lines: Lines<'static>) -> usize {
    lines.filter(|line| { is_valid(line) }).count()
}

fn is_valid(line: &str) -> bool {
    let v: Vec<i32> = line.split_whitespace().map(|num| { num.parse().unwrap() }).collect();
    let (a, b, c) = (v[0], v[1], v[2]);
    a < b + c && b < a + c && c < a + b
}

fn parse_file() -> Lines<'static> {
    include_str!("input/day3.txt").lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let valid_triangles = valid_count(parse_file());
        assert_eq!(917, valid_triangles);
    }
}
