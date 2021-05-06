fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-19 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let total: u32 = input.trim().parse().unwrap();

    let base_3 = (total as f64 - 1.0).log(3.0) as u32;
    let base_3 = 3_u32.pow(base_3);

    if total > 2 * base_3 {
        2 * total - 3 * base_3
    } else {
        total - base_3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        // Previous power of 3 is 3^1 == 3
        assert_eq!(1, solve_puzzle("4"));
        assert_eq!(2, solve_puzzle("5"));
        assert_eq!(3, solve_puzzle("6"));

        // Increases by 1 until double the previous power of 3
        assert_eq!(5, solve_puzzle("7"));
        assert_eq!(7, solve_puzzle("8"));
        assert_eq!(9, solve_puzzle("9"));

        // Pattern repeats at 3^2 == 9
        assert_eq!(1, solve_puzzle("10"));
        assert_eq!(2, solve_puzzle("11"));
        assert_eq!(9, solve_puzzle("18"));

        // Double the previous power == 18
        assert_eq!(11, solve_puzzle("19"));
        assert_eq!(13, solve_puzzle("20"));
        assert_eq!(27, solve_puzzle("27"));
    }
}
