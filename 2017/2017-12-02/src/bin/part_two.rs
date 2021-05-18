fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-02 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    input.lines().map(difference).sum()
}

fn difference(row: &str) -> u32 {
    let numbers: Vec<u32> = row.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    numbers.iter()
        .flat_map(|x| std::iter::repeat(x).zip(numbers.iter()))
        .find_map(|x| (x.0 % x.1 == 0 && x.0 != x.1).then(|| x.0 / x.1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(4, difference("5 9 2 8"));
        assert_eq!(3, difference("9 4 7 3"));
        assert_eq!(2, difference("3 8 6 5"));
    }
}
