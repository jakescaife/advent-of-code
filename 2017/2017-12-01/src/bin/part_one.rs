fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-01 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let digits: Vec<u32> = input.chars()
        .filter_map(|x| x.to_digit(10))
        .collect();

    let current = digits.iter();
    let compare = digits.iter().cycle().skip(1);

    current.zip(compare).filter(|x| x.0 == x.1).map(|x| x.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(3, solve_puzzle("1122"));
        assert_eq!(4, solve_puzzle("1111"));
        assert_eq!(0, solve_puzzle("1234"));
        assert_eq!(9, solve_puzzle("91212129"));
    }
}
