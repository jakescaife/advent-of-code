use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2018-02 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    input.lines().filter(|x| has_exact(x, 2)).count() *
        input.lines().filter(|x| has_exact(x, 3)).count()
}

fn has_exact(word: &str, count: usize) -> bool {
    let mut frequency = HashMap::new();
    word.chars()
        .for_each(|x| *frequency.entry(x).or_insert(0) += 1);
    frequency.iter().any(|x| *x.1 == count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let example = "abcdef\n\
                       bababc\n\
                       abbcde\n\
                       abcccd\n\
                       aabcdd\n\
                       abcdee\n\
                       ababab";
        assert_eq!(12, solve_puzzle(example));
    }
}
