advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let lines = input.lines();
    let mut list: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        list.push(numbers);
    }
    list
}

fn safe(list: &Vec<i32>) -> bool {
    let diff = list[0] - list[1];

    let direction: i32;
    if diff > 0 {
        direction = 1;
    } else {
        direction = -1;
    }

    for i in 1..list.len() {
        let diff = list[i-1] - list[i];

        match diff * direction {
            1 => {
                continue;
            }
            2 => {
                continue;
            }
            3 => {
                continue;
            }
            _ => {
                return false;
            }
        }
    }
    return true;
}

fn any_safe(list: &Vec<i32>) -> bool {
    let mut count = 0;
    for i in 0..list.len() {
        let mut new_list = list.clone();
        new_list.remove(i);
        if safe(&new_list) {
            count += 1;
        }
    }
    if count > 0 {
        return true;
    }
    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let processed_input = parse_input(input);
    let mut count = 0;
    for line in processed_input {
        if safe(&line){
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let processed_input = parse_input(input);
    let mut count = 0;
    for line in processed_input {
        if any_safe(&line){
            count += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
