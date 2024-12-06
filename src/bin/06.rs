use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn get_offset(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

#[derive(Clone)]
struct Guard {
    x: u32,
    y: u32,
    direction: Direction,
}

// Dirive clone
#[derive(Clone)]
struct State {
    map: Vec<Vec<bool>>,
    guard: Guard,
}

fn parse_input(input: &str) -> State {
    let lines = input.lines();
    let mut map = Vec::new();
    let mut guard = Guard {
        x: 0,
        y: 0,
        direction: Direction::Down,
    };
    for (y,line) in lines.into_iter().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().into_iter().enumerate() {
            match c {
                '#' => row.push(true),
                '.' => row.push(false),
                'V' => {
                    row.push(false);
                    guard = Guard {
                        x: x as u32,
                        y: y as u32,
                        direction: Direction::Down,
                    };
                },
                '>' => {
                    row.push(false);
                    guard = Guard {
                        x: x as u32,
                        y: y as u32,
                        direction: Direction::Right,
                    }

                },
                '<' => {
                    row.push(false);
                    guard = Guard {
                        x: x as u32,
                        y: y as u32,
                        direction: Direction::Left,
                    }
                },
                '^' => {
                    row.push(false);
                    guard = Guard {
                        x: x as u32,
                        y: y as u32,
                        direction: Direction::Up,
                    }
                },
                _ => panic!("Invalid character in input"),

            }
        }
        map.push(row);
    }
    State {
        map,
        guard
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut state = parse_input(input);
    let mut visited: HashSet<(u32,u32)> = HashSet::new();
    loop {
        visited.insert((state.guard.x, state.guard.y));
        let (dx, dy) = get_offset(&state.guard.direction);
        let x = state.guard.x as i32 + dx;
        let y = state.guard.y as i32 + dy;
        if x < 0 || y < 0 || x >= state.map[0].len() as i32 || y >= state.map.len() as i32 {
            break;
        }
        if state.map[y as usize][x as usize] {
            state.guard.direction = turn_right(&state.guard.direction);
        } else {
            state.guard.x = x as u32;
            state.guard.y = y as u32;

        }
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let state_original = parse_input(input);
    let mut state = state_original.clone();
    let mut visited: HashSet<(u32,u32)> = HashSet::new();
    loop {
        visited.insert((state.guard.x, state.guard.y));
        let (dx, dy) = get_offset(&state.guard.direction);
        let x = state.guard.x as i32 + dx;
        let y = state.guard.y as i32 + dy;
        if x < 0 || y < 0 || x >= state.map[0].len() as i32 || y >= state.map.len() as i32 {
            break;
        }
        if state.map[y as usize][x as usize] {
            state.guard.direction = turn_right(&state.guard.direction);
        } else {
            state.guard.x = x as u32;
            state.guard.y = y as u32;

        }
    }
    let mut count = 0;
    for visited in visited.iter() {
        let mut state = state_original.clone();
        state.map[visited.1 as usize][visited.0 as usize] = true;
        let mut visited: HashSet<(u32,u32, Direction)> = HashSet::new();
        loop {
            visited.insert((state.guard.x, state.guard.y, state.guard.direction.clone()));
            let (dx, dy) = get_offset(&state.guard.direction);
            let x = state.guard.x as i32 + dx;
            let y = state.guard.y as i32 + dy;
            if x < 0 || y < 0 || x >= state.map[0].len() as i32 || y >= state.map.len() as i32 {
                break;
            }
            if visited.contains(&(x as u32, y as u32, state.guard.direction.clone())) {
                count += 1;
                break;
            }
            if state.map[y as usize][x as usize] {
                state.guard.direction = turn_right(&state.guard.direction);
            } else {
                state.guard.x = x as u32;
                state.guard.y = y as u32;

            }
        }
    }
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
