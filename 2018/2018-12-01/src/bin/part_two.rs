use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2018-01 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    let mut visited = HashSet::new();
    let mut frequencies = input.lines().cycle()
        .map(|x| x.parse().unwrap())
        .scan(0, |s, x: i32| { *s += x; Some(*s) });
    frequencies.find(|x| !visited.insert(*x)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(solve_puzzle("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(solve_puzzle("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(solve_puzzle("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
