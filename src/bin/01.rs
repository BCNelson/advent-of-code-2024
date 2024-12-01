use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();
    // get each list of numbers
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in lines {
        // spit the line into two numbers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if numbers.len() != 2 {
            panic!("Invalid input");
        }
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }
    (list1, list2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = parse_input(input);
    // sort the lists
    list1.sort();
    list2.sort();

    if list1.len() != list2.len() {
        panic!("Invalid input list different lengths");
    }

    let mut sum: i32 = 0;
    for i in 0..list1.len() {
        let distance = list1[i] - list2[i];
        // if the distance is negative, we need to add the absolute value
        if distance < 0 {
            sum += distance.abs();
        } else {
            sum += distance;
        }
    }
    Some(sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse_input(input);
    let mut map: HashMap<i32, u32> = HashMap::new();

    for i in 0..list2.len() {
        let item = list2[i];
        // if the item is not in the map, add it
        match map.get_mut(&item) {
            Some(count) => {
                *count += 1;
            }
            None => {
                map.insert(item, 1);
            }
        }
    }

    let mut sum: u32 = 0;
    for i in 0..list1.len() {
        let item = list1[i];
        // if the item is not in the map, add it
        match map.get(&item) {
            Some(count) => {
                sum += (item as u32) * count;
            }
            None => {
                sum += 0;
            }
        }
    }
    Some(sum)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
