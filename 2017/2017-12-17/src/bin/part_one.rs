fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-17 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let steps: usize = input.trim().parse().unwrap();
    let mut buffer = vec![0];
    let mut index = 0;

    for length in 1..2018 {
        index = (index + steps + 1) % length;
        buffer.insert(index + 1, length);
    }

    buffer[index + 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(638, solve_puzzle("3"));
    }
}
