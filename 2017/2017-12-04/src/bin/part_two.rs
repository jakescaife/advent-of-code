use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-04 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    input.lines().filter(|x| accepted(x)).count()
}

fn accepted(phrase: &str) -> bool {
    let mut words = HashSet::new();
    phrase.split_whitespace().map(sorted_word).all(|x| words.insert(x))
}

fn sorted_word(word: &str) -> String {
    let mut characters: Vec<_> = word.chars().collect();
    characters.sort_unstable();
    characters.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert!(accepted("abcde fghij"));
        assert!(accepted("a ab abc abd abf abj"));
        assert!(accepted("iiii oiii ooii oooi oooo"));
        assert!(!accepted("abcde xyz ecdab"));
        assert!(!accepted("oiii ioii iioi iiio"));
    }
}
