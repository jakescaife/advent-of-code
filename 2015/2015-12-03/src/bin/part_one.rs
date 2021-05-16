use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-03 Part One: {}", solve_puzzle(&input));
}

type Position = (i32, i32);

fn solve_puzzle(input: &str) -> usize {
    let mut visited_houses: HashSet<_> = input.chars()
        .scan((0, 0), |s, x| { *s = next_house(s, x); Some(*s) })
        .collect();

    visited_houses.insert((0, 0));
    visited_houses.len()
}

fn next_house(position: &Position, direction: char) -> Position {
    match direction {
        '^' => (position.0 + 1, position.1),
        'v' => (position.0 - 1, position.1),
        '>' => (position.0, position.1 + 1),
        '<' => (position.0, position.1 - 1),
        _ => panic!("Movement direction unrecognised"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(2, solve_puzzle(">"));
        assert_eq!(4, solve_puzzle("^>v<"));
        assert_eq!(2, solve_puzzle("^v^v^v^v^v"));
    }
}
