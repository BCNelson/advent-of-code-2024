advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // match all insances of regex
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let ret = re.captures_iter(input).fold(Some(0), |acc, cap| {
        let a: u32 = cap[1].parse().unwrap();
        let b: u32 = cap[2].parse().unwrap();
        acc.map(|acc| acc + a * b)
    });
    ret
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parts = input.split("don't()");
    let first = parts.next().unwrap();
    let mut acc = 0;
    acc += part_one(first).unwrap();
    for part in parts {
        let mut secPart = part.split("do()");
        secPart.next(); // skip first
        for p in secPart {
            acc += part_one(p).unwrap();
        }
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
