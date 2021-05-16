fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-02 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    input.lines().map(calculate).sum()
}

fn calculate(present: &str) -> usize {
    let mut dimensions: Vec<usize> = present.split('x')
        .map(|x| x.parse().unwrap())
        .collect();

    dimensions.sort_unstable();

    3 * dimensions[0] * dimensions[1]
        + 2 * dimensions[1] * dimensions[2]
        + 2 * dimensions[0] * dimensions[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(58, calculate("2x3x4"));
        assert_eq!(43, calculate("1x1x10"));
    }
}
