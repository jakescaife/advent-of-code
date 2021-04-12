fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-01 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    let mut direction = (0, 1);

    for instruction in input.trim().split(", ") {
        direction = match instruction.chars().next() {
            Some('L') => (-direction.1, direction.0),
            Some('R') => (direction.1, -direction.0),
            _ => panic!("Unexpected instruction"),
        };

        let distance: i32 = instruction[1..].parse().unwrap();

        position = (
            position.0 + direction.0 * distance,
            position.1 + direction.1 * distance,
        );
    }

    position.0.abs() + position.1.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(5, solve_puzzle("R2, L3"));
        assert_eq!(2, solve_puzzle("R2, R2, R2"));
        assert_eq!(12, solve_puzzle("R5, L5, R5, R3"));
    }
}
