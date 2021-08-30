fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2018-01 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    input.lines().map(|x| x.parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(solve_puzzle("+1\n+1\n+1"),  3);
        assert_eq!(solve_puzzle("+1\n+1\n-2"),  0);
        assert_eq!(solve_puzzle("-1\n-2\n-3"), -6);
    }
}
