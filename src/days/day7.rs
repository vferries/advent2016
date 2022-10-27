use std::str::Lines;

use fancy_regex::Regex;

pub fn day7() {
    println!("=== Day 7 ===");
    println!("- Part 1");
    let mut lines = parse_file();
    println!("{}", part1(&mut lines));
    println!("- Part 2");
    let mut lines = parse_file();
    println!("{}", part2(&mut lines));
}

pub fn part1(lines: &mut Lines) -> usize {
    lines.filter(|s| {supports_tls(&s)}).count()
}

pub fn part2(lines: &mut Lines) -> usize {
    lines.filter(|s| {supports_ssl(&s)}).count()
}

fn parse_file() -> Lines<'static> {
    include_str!("input/day7.txt").lines()
}

fn supports_tls(ipv7: &str) -> bool {
    let parts: Vec<&str> = ipv7.split(|c| c == '[' || c == ']').collect();
    let parts = parts
        .into_iter()
        .map(contains_abba)
        .enumerate();
    let (base, brackets): (Vec<_>, Vec<_>) = parts.partition(|(i, _)| { i % 2 == 0 });
    base.into_iter().any(|(_, b)| { b }) && brackets.into_iter().all(|(_, b)| { !b })
}

fn contains_abba(text: &str) -> bool {
    let re = Regex::new(r"(.)(?!\1)(.)\2\1").unwrap();
    re.is_match(text).unwrap()
}

fn supports_ssl(text: &str) -> bool {
    let re1 = Regex::new(r"(?:^|])[^\[]*(.)(?!\1)(.)\1.*\[[^]]*\2\1\2").unwrap();
    let re2 = Regex::new(r"\[[^]]*(.)(?!\1)(.)\1.*][^\[]*\2\1\2").unwrap();
    let result1 = re1.is_match(text).unwrap();
    let result2 = re2.is_match(text).unwrap();
    println!("{} {}", result1, result2);
    result1 || result2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_contains_abba() {
        assert_eq!(true, contains_abba("xabbay"));
    }

    #[test]
    fn part1_abba_random_string() {
        assert_eq!(false, contains_abba("xboczapbuezci"));
    }

    #[test]
    fn part1_abba_same_letter() {
        assert_eq!(false, contains_abba("xaaaay"));
    }

    #[test]
    fn part1_sample1() {
        assert_eq!(true, supports_tls("abba[mnop]qrst"));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!(false, supports_tls("abcd[bddb]xyyx"));
    }

    #[test]
    fn part1_sample3() {
        assert_eq!(false, supports_tls("aaaa[qwer]tyui"));
    }

    #[test]
    fn part1_sample4() {
        assert_eq!(true, supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn part1_real() {
        let mut lines = include_str!("input/day7.txt").lines();
        assert_eq!(118, part1(&mut lines));
    }

    #[test]
    fn part2_sample1() {
        assert_eq!(true, supports_ssl("aba[bab]xyz"));
    }

    #[test]
    fn part2_sample2() {
        assert_eq!(false, supports_ssl("xyx[xyx]xyx"));
    }

    #[test]
    fn part2_sample3() {
        assert_eq!(true, supports_ssl("aaa[kek]eke"));
    }

    #[test]
    fn part2_sample4() {
        assert_eq!(true, supports_ssl("zazbz[bzb]cdb"));
    }

    #[test]
    fn part2_sample5() {
        assert_eq!(true, supports_ssl("aaa[xyx]bbb[ccc]ddd[eee]yxy"));
    }

    #[test]
    fn part2_result() {
        let mut lines = include_str!("input/day7.txt").lines();
        assert_eq!(260, part2(&mut lines));
    }
}
