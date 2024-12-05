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
    None
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
        assert_eq!(result, None);
    }
}
