fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-04 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let input = input.trim();
    (0..).find(|x| has_zeroes(input, *x)).unwrap()
}

fn has_zeroes(word: &str, number: usize) -> bool {
    let digest = md5::compute(format!("{}{}", word, number));
    digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(609043, solve_puzzle("abcdef"));
        assert_eq!(1048970, solve_puzzle("pqrstuv"));
    }
}
