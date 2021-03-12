fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-05 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    input
        .lines()
        .filter(|x| three_vowels(x))
        .filter(|x| repeated_letter(x))
        .filter(|x| unwanted_strings(x))
        .count()
}

fn three_vowels(word: &str) -> bool {
    word.chars().filter(|&x| "aeiou".contains(x)).count() > 2
}

fn repeated_letter(word: &str) -> bool {
    word.chars().zip(word.chars().skip(1)).any(|x| x.0 == x.1)
}

fn unwanted_strings(word: &str) -> bool {
    ["ab", "cd", "pq", "xy"].iter().all(|x| !word.contains(x))
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
