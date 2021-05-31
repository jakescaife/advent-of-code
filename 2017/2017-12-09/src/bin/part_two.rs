fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-09 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let mut input = input.chars();

    let mut waste = false;
    let mut score = 0;

    while let Some(character) = input.next() {
        match character {
            '!' => { input.next(); }
            '>' => waste = false,
            _ if waste => score += 1,
            '<' => waste = true,
            _ => continue,
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(0, solve_puzzle("<>"));
        assert_eq!(3, solve_puzzle("<<<<>"));
        assert_eq!(2, solve_puzzle("<{!>}>"));
        assert_eq!(0, solve_puzzle("<!!>"));
        assert_eq!(0, solve_puzzle("<!!!>>"));
    }
}
