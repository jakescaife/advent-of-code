fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-15 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let discs: Vec<_> = input.lines()
        .map(parse_disc)
        .scan(0, |s, x| { *s += 1; Some((*s, x.0, x.1)) })
        .collect();

    (0..).find(|t| discs.iter().all(|x| (t + x.0 + x.2) % x.1 == 0)).unwrap()
}

fn parse_disc(description: &str) -> (usize, usize) {
    let description: Vec<_> = description
        .trim_end_matches('.')
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    (description[0], description[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = "Disk #1 has 5 positions; at time=0, it is at position 4.\n\
                    Disk #2 has 2 positions; at time=0, it is at position 1.";
        assert_eq!(5, solve_puzzle(&data));
    }
}
