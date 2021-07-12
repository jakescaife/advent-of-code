fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-05 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    input.lines()
        .filter(|x| vowels(x) >= 3)
        .filter(|x| repeat(x))
        .filter(|x| !unwanted(x))
        .count()
}

fn vowels(word: &str) -> usize {
    word.chars().filter(|x| "aeiou".contains(*x)).count()
}

fn repeat(word: &str) -> bool {
    word.as_bytes().windows(2).any(|x| x[0] == x[1])
}

fn unwanted(word: &str) -> bool {
    ["ab", "cd", "pq", "xy"].iter().any(|x| word.contains(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(1, solve_puzzle("ugknbfddgicrmopn"));
        assert_eq!(1, solve_puzzle("aaa"));
        assert_eq!(0, solve_puzzle("jchzalrnumimnmhp"));
        assert_eq!(0, solve_puzzle("haegwjzuvuyypxyu"));
        assert_eq!(0, solve_puzzle("dvszwmarrgswjxmb"));
    }
}
