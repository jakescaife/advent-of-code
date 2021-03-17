fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-08 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.escape_default().count() + 2 - x.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let strings = "\"\"\n\
                       \"abc\"\n\
                       \"aaa\\\"aaa\"\n\
                       \"\\x27\"";

        assert_eq!(19, solve_puzzle(strings));
    }
}
