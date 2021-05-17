fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-01 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let digits: Vec<u32> = input.chars()
        .filter_map(|x| x.to_digit(10))
        .collect();

    let current = digits.iter();
    let compare = digits.iter().cycle().skip(digits.len() / 2);

    current.zip(compare).filter(|x| x.0 == x.1).map(|x| x.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(6, solve_puzzle("1212"));
        assert_eq!(0, solve_puzzle("1221"));
        assert_eq!(4, solve_puzzle("123425"));
        assert_eq!(4, solve_puzzle("12131415"));
    }
}
