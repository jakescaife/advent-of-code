fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-13 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let scanners = input.lines().map(extract_details).collect();
    (0..).find(|t| undetected(*t, &scanners)).unwrap()
}

fn undetected(time: u32, scanners: &Vec<(u32, u32)>) -> bool {
    scanners.iter().all(|x| (time + x.0) % (2 * (x.1 - 1)) != 0)
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
        assert_eq!(10, solve_puzzle("0: 3\n1: 2\n4: 4\n6: 4"));
    }
}
