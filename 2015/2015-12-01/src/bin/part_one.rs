fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-01 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    input.chars().map(|x| if x == '(' { 1 } else { -1 }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(0, solve_puzzle("(())"));
        assert_eq!(3, solve_puzzle("((("));
    }
}
