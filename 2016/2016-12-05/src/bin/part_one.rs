fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-05 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let input = input.trim();
    (0..).filter_map(|x| next_character(input, x)).take(8).collect()
}

fn next_character(word: &str, number: usize) -> Option<char> {
    let key = format!("{}{}", word, number);
    let hash = format!("{:?}", md5::compute(key));
    hash.starts_with("00000").then(|| hash.chars().nth(5).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!("18f47a30", &solve_puzzle("abc"));
    }
}
