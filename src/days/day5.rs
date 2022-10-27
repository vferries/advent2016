
pub fn day5() {
    println!("=== Day 5 ===");
    println!("- Part 1");
    let id = parse_file();
    println!("{}", part1(id));
    println!("- Part 2");
    println!("{}", part2(id));
}

pub fn part1(id: &str) -> String {
    let mut result = String::new();
    let mut index = 0;
    loop {
        let current = String::from(id) + index.to_string().as_str();
        let digest = md5::compute(current.as_bytes());
        let bytes = digest.0;
        if bytes[0] == 0 && bytes[1] == 0 && bytes[2] < 16 {
            println!("Found one");
            let sixth = format!("{:x}", digest).chars().nth(5).unwrap();
            result.push(sixth);
            if result.len() >= 8 {
                break;
            }
        }
        index += 1;
    }
    result
}

pub fn part2(id: &str) -> String {
    let mut result: [char; 8] = [' '; 8];
    let mut index = 0;
    loop {
        let current = String::from(id) + index.to_string().as_str();
        let digest = md5::compute(current.as_bytes());
        let bytes = digest.0;
        if bytes[0] == 0 && bytes[1] == 0 && bytes[2] < 8 {
            let position: usize = bytes[2] as usize;
            let char = format!("{:x}", digest).chars().nth(6).unwrap();
            if result[position] == ' ' {
                result[position] = char;
                let result = result.map(|c| {c.to_string()}).join("");
                if !result.contains(" ") {
                    return result;
                }
            }
        }
        index += 1;
    }
}

fn parse_file() -> &'static str {
    include_str!("input/day5.txt").lines().next().unwrap()
}

#[cfg(test)]
mod tests {
/*
    use super::*;
    #[test]
    fn part1_sample1() {
        assert_eq!("18f47a30", part1("abc"));
    }

    #[test]
    fn part1_real() {
        assert_eq!("1a3099aa", part1("uqwqemis"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!("05ace8e3", part1("abc"));
    }

    #[test]
    fn part2_real() {
        assert_eq!("694190cd", part1("uqwqemis"));
    }*/
}
