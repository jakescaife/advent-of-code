fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-04 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let input = input.trim();
    (0..).find(|x| has_zeroes(input, *x)).unwrap()
}

fn has_zeroes(word: &str, number: usize) -> bool {
    let key = format!("{}{}", word, number);
    format!("{:?}", md5::compute(key)).get(..5) == Some("00000")
}

#[cfg(test)]
mod tests {
    use super::*;

    // It can take a while for this to run, so we can split the examples into seperate functions
    // to run them concurrently.

    #[test]
    fn example_one() {
        assert_eq!(609043, solve_puzzle("abcdef"));
    }

    #[test]
    fn example_two() {
        assert_eq!(1048970, solve_puzzle("pqrstuv"));
    }
}
