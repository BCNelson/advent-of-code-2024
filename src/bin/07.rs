advent_of_code::solution!(7);

struct Equation {
    answer: i64,
    params: Vec<i64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    let lines = input.lines();
    let mut equations = Vec::new();
    for line in lines {

        let mut parts = line.split(": ");
        let answer = parts.next().unwrap().parse().unwrap();

        let params = parts.next().unwrap().split(" ");
        let params = params.map(|x| x.parse().unwrap()).collect();
        equations.push(Equation { answer, params });
    }
    equations
}

fn solve_equation(sum: i64, equation: &Equation, depth: usize) -> bool {
    if depth == equation.params.len() {
        return sum == equation.answer;
    }

    if solve_equation(sum + equation.params[depth], &equation, depth + 1) {
        return true;
    }

    if solve_equation(sum * equation.params[depth], &equation, depth + 1) {
        return true;
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let equations = parse_input(input);
    let mut count = 0;
    for (i, equation) in equations.into_iter().enumerate() {
        if solve_equation(equation.params[0], &equation, 1) {
            count += equation.answer;
        }
    }
    Some(count as u32)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
