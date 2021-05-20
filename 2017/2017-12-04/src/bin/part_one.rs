use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-04 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    input.lines().filter(|x| accepted(x)).count()
}

fn accepted(phrase: &str) -> bool {
    let mut words = HashSet::new();
    phrase.split_whitespace().all(|x| words.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert!(accepted("aa bb cc dd ee"));
        assert!(accepted("aa bb cc dd aaa"));
        assert!(!accepted("aa bb cc dd aa"));
    }
}
