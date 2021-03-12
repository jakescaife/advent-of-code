fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-05 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    input
        .lines()
        .filter(|x| two_pairs(x))
        .filter(|x| sandwich(x))
        .count()
}

fn two_pairs(word: &str) -> bool {
    word.chars()
        .zip(word.chars().skip(1))
        .map(|x| format!("{}{}", x.0, x.1))
        .any(|x| word.matches(&x).count() > 1)
}

fn sandwich(word: &str) -> bool {
    word.chars().zip(word.chars().skip(2)).any(|x| x.0 == x.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(1, solve_puzzle("qjhvhtzxzqqjkmpb"));
        assert_eq!(1, solve_puzzle("xxyxx"));
        assert_eq!(0, solve_puzzle("uurcxstgmygtbstg"));
        assert_eq!(0, solve_puzzle("ieodomkazucvgmuy"));
    }
}
