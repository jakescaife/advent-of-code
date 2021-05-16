fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-01 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let mut accumulator = input.chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .scan(0, |s, x| { *s += x; Some(*s) });
    accumulator.position(|x| x < 0).map(|x| x + 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(1, solve_puzzle(")"));
        assert_eq!(5, solve_puzzle("()())"));
    }
}
