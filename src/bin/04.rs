advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines();
    let mut list: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        list.push(chars);
    }
    list
}

static DIRECTIONS: [(i32, i32, &str); 8] = [
    (0, 1, "right"), // right
    (1, 1, "down right"), // down right
    (1, 0, "down"), // down
    (1, -1, "down left"), // down left
    (0, -1, "left"), // left
    (-1, -1, "up left"), // up left
    (-1, 0, "up"), // up
    (-1, 1, "up right") // up right
];

static WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn check_position(list: Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    if list[i as usize][j as usize] != WORD[0] {
        return count;
    }
    'outer: for direction in DIRECTIONS.iter() {
        let mut x = i as i32;
        let mut y = j as i32;
        for pos in 1..WORD.len() {
            x += direction.0;
            y += direction.1;
            if x < 0 || y < 0 || x >= list.len() as i32 || y >= list[0].len() as i32 {
                continue 'outer;
            }
            if list[x as usize][y as usize] != WORD[pos] {
                continue 'outer;
            }
        }
        println!("Found word at ({}, {}) {}\n", i, j, direction.2);
        count += 1;
    }
    count
}

const CORNERS: [(i32, i32, &str); 4] = [
    (1, 1, "down right"), // down right
    (1, -1, "down left"), // down left
    (-1, -1, "up left"), // up left
    (-1, 1, "up right") // up right
];

fn check_position2(list: Vec<Vec<char>>, i: i32, j: i32) -> bool {
    let mut count = 0;
    for corner in CORNERS.iter() {
        let x: i32 = corner.0.try_into().unwrap();
        let y: i32 = corner.1.try_into().unwrap();
        let index1 = i + x;
        let index2 = j + y;
        if index1 < 0 || index2 < 0 || index1 as usize >= list.len() || index2 as usize >= list[0].len() {
            continue;
        }
        let char = list[index1 as usize][index2 as usize];
        match char {
            'M' => {
                let index1 = i + (-1*x);
                let index2 = j + (-1*y);
                if index1 < 0 || index2 < 0 || index1 as usize >= list.len() || index2 as usize >= list[0].len() {
                    continue;
                }
                if list[index1 as usize][index2 as usize] == 'S' {
                    count += 1;
                }
            }
            _ => {}
        }
    }
    count == 2
}

pub fn part_one(input: &str) -> Option<u32> {
    let processed_input = parse_input(input);
    let mut count = 0;
    for line in processed_input.iter().enumerate() {
        for (j, c) in line.1.iter().enumerate() {
            if *c == 'X' {
                count += check_position(processed_input.clone(), line.0, j);
            }
        }
    }
    Some(count.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let processed_input = parse_input(input);
    let mut count = 0;
    for line in processed_input.iter().enumerate() {
        for (j, c) in line.1.iter().enumerate() {
            if *c == 'A' {
                if check_position2(processed_input.clone(), line.0 as i32, j as i32) {
                    count += 1;
                }
            }
        }
    }
    Some(count.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
