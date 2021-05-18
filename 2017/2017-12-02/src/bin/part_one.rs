fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-02 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    input.lines().map(difference).sum()
}

fn difference(row: &str) -> u32 {
    let numbers: Vec<u32> = row.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(8, difference("5 1 9 5"));
        assert_eq!(4, difference("7 5 3"));
        assert_eq!(6, difference("2 4 6 8"));
    }
}
