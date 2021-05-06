fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-19 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let total: u32 = input.trim().parse().unwrap();
    let significant_digit = 0x80000000 >> total.leading_zeros();
    let remainder = total - significant_digit;
    remainder * 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(3, solve_puzzle("5"));
    }
}
