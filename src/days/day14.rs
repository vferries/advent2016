use std::collections::HashMap;
use std::ops::Index;

// TODO We should rather start with quintuples and verify if there are triples matching those.

pub fn day14() {
    let seed = "qzyelonm";
    let result = part1(seed);
    println!("Part 1 = {}", result);

    let result = part2(seed);
    println!("Part 2 = {}", result);
}

fn is_key(seed: &str, index: usize, memo: &mut HashMap<usize, String>) -> bool {
    let current_md5 = compute_md5(seed, index, memo);
    let triple = find_triple(current_md5);
    if let Some(triple) = triple {
        let quintuple = format!("{}", triple).repeat(5);
        for i in index + 1..=index + 1001 {
            let result_str = compute_md5(seed, i, memo);
            if result_str.contains(&quintuple) {
                return true;
            }
        }
    }
    false
}

fn is_stretched_key(seed: &str, index: usize, memo: &mut HashMap<usize, String>) -> bool {
    let current_md5 = compute_stretched_md5(seed, index, memo);
    let triple = find_triple(current_md5);
    if let Some(triple) = triple {
        let quintuple = format!("{}", triple).repeat(5);
        for i in index + 1..=index + 1001 {
            let result_str = compute_stretched_md5(seed, i, memo);
            if result_str.contains(&quintuple) {
                return true;
            }
        }
    }
    false
}

fn compute_md5<'a>(seed: &'a str, index: usize, memo: &'a mut HashMap<usize, String>) -> &'a str {
    if !memo.contains_key(&index) {
        let current = format!("{}{}", seed, index);
        let digest = md5::compute(current);
        let current_md5 = format!("{:x}", digest);
        memo.insert(index, current_md5);
    }
    return memo.index(&index);
}

fn compute_stretched_md5<'a>(seed: &'a str, index: usize, memo: &'a mut HashMap<usize, String>) -> &'a str {
    if !memo.contains_key(&index) {
        let mut current = format!("{}{}", seed, index);
        for _ in 0..=2016 {
            let digest = md5::compute(current);
            current = format!("{:x}", digest);
        }
        memo.insert(index, current);
    }
    return memo.index(&index);
}

fn find_triple(str: &str) -> Option<char> {
    let mut previous: Option<char> = None;
    let mut count = 0;
    for c in str.chars() {
        if previous.filter(|p| *p == c).is_none() {
            count = 1;
            previous = Some(c);
        } else {
            count += 1;
        }
        if count == 3 {
            return Some(c);
        }
    }
    None
}

fn part1(salt: &str) -> usize {
    let mut found = 0;
    let mut i = 0;
    let memo = &mut HashMap::new();
    while found < 64 {
        if is_key(salt, i, memo) {
            found += 1;
        }
        i += 1
    }
    i - 1
}

fn part2(salt: &str) -> usize {
    let mut found = 0;
    let mut i = 0;
    let memo = &mut HashMap::new();
    while found < 64 {
        if is_stretched_key(salt, i, memo) {
            found += 1;
            println!("{}", found);
        }
        i += 1
    }
    i - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(false, is_key("abc", 18, &mut HashMap::new()));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!(true, is_key("abc", 39, &mut HashMap::new()));
    }

    #[test]
    fn part1_sample3() {
        assert_eq!(false, is_key("abc", 40, &mut HashMap::new()));
        assert_eq!(false, is_key("abc", 41, &mut HashMap::new()));
        assert_eq!(false, is_key("abc", 42, &mut HashMap::new()));
        assert_eq!(false, is_key("abc", 43, &mut HashMap::new()));
        assert_eq!(false, is_key("abc", 44, &mut HashMap::new()));
        assert_eq!(false, is_key("abc", 45, &mut HashMap::new()));
    }

    #[test]
    fn part1_sample4() {
        assert_eq!(true, is_key("abc", 92, &mut HashMap::new()));
    }

    #[test]
    fn part1_sample6() {
        assert_eq!(true, is_key("abc", 22728, &mut HashMap::new()));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(22728, part1("abc"));
    }

    #[test]
    fn part1_input() {
        let salt = "qzyelonm";
        assert_eq!(15168, part1(salt));
    }
}
