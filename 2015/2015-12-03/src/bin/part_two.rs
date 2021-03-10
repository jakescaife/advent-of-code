use std::collections::HashSet;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-03 Part Two: {}", puzzle_solution);
}

type Position = (i32, i32);

fn solve_puzzle(input: &str) -> usize {
    let santa = input
        .chars()
        .step_by(2)
        .scan((0, 0), |s, x| Some(next(s, x)));

    let robot = input
        .chars()
        .skip(1)
        .step_by(2)
        .scan((0, 0), |s, x| Some(next(s, x)));

    let visited_houses: HashSet<_> = santa.chain(robot).chain(std::iter::once((0, 0))).collect();

    visited_houses.len()
}

fn next(position: &mut Position, direction: char) -> Position {
    match direction {
        '^' => position.0 += 1,
        'v' => position.0 -= 1,
        '>' => position.1 += 1,
        '<' => position.1 -= 1,
        _ => panic!("Unexpected direction."),
    }

    *position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(3, solve_puzzle("^v"));
        assert_eq!(3, solve_puzzle("^>v<"));
        assert_eq!(11, solve_puzzle("^v^v^v^v^v"));
    }
}
