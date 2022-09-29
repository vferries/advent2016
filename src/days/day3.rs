use std::str::Lines;

pub fn day3() {
    println!("=== Day 3 ===");
    println!("- Part 1");
    let lines = parse_file();
    println!("{}", valid_count(lines));
    println!("- Part 2");
    let lines = parse_file();
    println!("{}", valid_count_2(lines));
}

fn valid_count(lines: Lines<'static>) -> usize {
    lines.filter(|line| is_line_valid(line)).count()
}

fn valid_count_2(lines: Lines<'static>) -> usize {
    let lines: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    lines
        .chunks(3)
        .flat_map(move |slice| {
            let (v1, v2, v3) = (
                slice.get(0).unwrap(),
                slice.get(1).unwrap(),
                slice.get(2).unwrap(),
            );
            let (a1, b1, c1) = (v1[0], v1[1], v1[2]);
            let (a2, b2, c2) = (v2[0], v2[1], v2[2]);
            let (a3, b3, c3) = (v3[0], v3[1], v3[2]);
            vec![(a1, a2, a3), (b1, b2, b3), (c1, c2, c3)]
        })
        .filter(|(a, b, c)| is_valid(*a, *b, *c))
        .count()
}

fn is_line_valid(line: &str) -> bool {
    let v: Vec<i32> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let (a, b, c) = (v[0], v[1], v[2]);
    is_valid(a, b, c)
}

fn is_valid(a: i32, b: i32, c: i32) -> bool {
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

    #[test]
    fn part2() {
        let valid_triangles = valid_count_2(parse_file());
        assert_eq!(1649, valid_triangles);
    }
}
