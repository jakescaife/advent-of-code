fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-05 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    input.lines().filter(|x| two_pairs(x) && sandwich(x)).count()
}

fn two_pairs(word: &str) -> bool {
    word.as_bytes().windows(2)
        .map(|x| format!("{}{}", x[0] as char, x[1] as char))
        .any(|x| word.matches(&x).count() > 1)
}

fn sandwich(word: &str) -> bool {
    word.as_bytes().windows(3).any(|x| x[0] == x[2])
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
