use itertools::Itertools;

pub fn day18() {
    let result = part1(include_str!("input/day18.txt"), 40);
    println!("Part 1 = {}", result);
    let result = part1(include_str!("input/day18.txt"), 400_000);
    println!("Part 2 = {}", result);
}

fn part1(input: &str, rows: usize) -> usize {
    let mut row = String::from(input);
    let mut safe_tiles = 0;
    for _ in 0..rows {
        safe_tiles += row.chars().filter(|c| *c == '.').count();
        row = next(row);
    }
    safe_tiles
}

fn next(line: String) -> String {
    let expanded = format!(".{}.", line);
    line.char_indices().map(|(i, _)| {
        let top = &expanded[i..=i+2];
        if top == "^^." || top == ".^^" || top == "^.." || top == "..^" {
            '^'
        } else {
            '.'
        }
    }).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(".^^^^", next(String::from("..^^.")));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1974, part1(include_str!("input/day18.txt"), 40));
    }

    #[test]
    fn part2_input() {
        assert_eq!(19991126, part1(include_str!("input/day18.txt"), 400_000));
    }
}
