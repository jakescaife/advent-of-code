use std::collections::HashSet;
use std::iter;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-01 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    let mut direction = (0, 1);
    let mut visited: HashSet<_> = iter::once(position).collect();

    for instruction in input.trim().split(", ") {
        direction = match instruction.chars().next() {
            Some('L') => (-direction.1, direction.0),
            Some('R') => (direction.1, -direction.0),
            _ => panic!("Unexpected instruction"),
        };

        for _ in 0..instruction[1..].parse().unwrap() {
            position = (position.0 + direction.0, position.1 + direction.1);
            if !visited.insert(position) {
                return position.0.abs() + position.1.abs();
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(4, solve_puzzle("R8, R4, R4, R8"));
    }
}
