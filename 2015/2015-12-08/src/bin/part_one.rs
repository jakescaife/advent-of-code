fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-08 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    input
        .lines()
        .map(|x| {
            let s = x.replace("\\\\", "-");
            let s = s.replace("\\\"", "-");
            x.len() - (s.len() - 2 - 3 * s.matches("\\x").count())
        })
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

        assert_eq!(12, solve_puzzle(strings));
    }
}
