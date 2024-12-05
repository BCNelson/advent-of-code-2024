use core::num;
use std::{
    collections::{HashMap, HashSet},
    vec,
};
advent_of_code::solution!(5);

#[derive(Debug)]
struct Input {
    orderings: HashMap<u32, (HashSet<u32>, HashSet<u32>)>, // (before, after)
    updates: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> Input {
    let mut parsed = Input {
        orderings: HashMap::new(),
        updates: Vec::new(),
    };
    let mut lines = input.lines();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let mut parts = line.split("|");
        let part1 = parts.next().unwrap().parse().unwrap();
        let part2 = parts.next().unwrap().parse().unwrap();
        let part1_hash = parsed.orderings.get_mut(&part1);
        match part1_hash {
            Some((a, _)) => {
                a.insert(part2);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(part2);
                parsed.orderings.insert(part1, (set, HashSet::new()));
            }
        }
        let part2_hash = parsed.orderings.get_mut(&part2);
        match part2_hash {
            Some((_, b)) => {
                b.insert(part1);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(part1);
                parsed.orderings.insert(part2, (HashSet::new(), set));
            }
        }
    }
    loop {
        let line = lines.next();
        match line {
            None => {
                break;
            }
            Some(line) => {
                let parts = line
                    .split(",")
                    .map(|s| s.parse::<u32>())
                    .map(Result::unwrap);
                parsed.updates.push(parts.collect());
            }
        }
    }
    parsed
}

fn validate_update(
    update: Vec<u32>,
    orderings: &HashMap<u32, (HashSet<u32>, HashSet<u32>)>,
) -> bool {
    for page in update.iter().enumerate() {
        let rules = orderings.get(page.1);
        match rules {
            Some((before, after)) => {
                for i in 0..page.0 {
                    if orderings.contains_key(&update[i]) {
                        if !after.contains(&update[i]) {
                            return false;
                        }
                    }
                }
                for i in page.0 + 1..update.len() {
                    if orderings.contains_key(&update[i]) {
                        if !before.contains(&update[i]) {
                            return false;
                        }
                    }
                }
            }
            None => {
                continue;
            }
        }
    }
    true
}

fn get_middle(vec: Vec<u32>) -> u32 {
    vec[vec.len() / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse_input(input);
    println!("{:?}", parsed);
    let mut count = 0;
    for update in parsed.updates.iter().enumerate() {
        if validate_update(update.1.clone(), &parsed.orderings) {
            count += get_middle(update.1.clone());
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = parse_input(input);
    println!("{:?}", parsed);
    let mut count = 0;
    for update in parsed.updates.iter().enumerate() {
        if !validate_update(update.1.clone(), &parsed.orderings) {
            let mut sorted_update = update.1.clone();
            sorted_update.sort_by(|a, b| {
                let a_rules = parsed.orderings.get(a);
                match a_rules {
                    Some((_, bh)) => {
                        if bh.contains(b) {
                            return std::cmp::Ordering::Less;
                        } else {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                    None => {
                        println!("No rules for {}", a);
                        return std::cmp::Ordering::Equal;
                    }
                }
            });
            println!("{:?}", sorted_update);
            count += get_middle(sorted_update);
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
