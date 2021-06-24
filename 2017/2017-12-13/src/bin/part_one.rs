fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-13 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    input.lines()
        .map(extract_details)
        .filter(|x| x.0 % (2 * (x.1 - 1)) == 0)
        .fold(0, |s, x| s + x.0 * x.1)
}

fn extract_details(line: &str) -> (u32, u32) {
    let mut line = line.split(": ");
    let depth = line.next().and_then(|x| x.parse().ok()).unwrap();
    let range = line.next().and_then(|x| x.parse().ok()).unwrap();
    (depth, range)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(24, solve_puzzle("0: 3\n1: 2\n4: 4\n6: 4"));
    }
}
