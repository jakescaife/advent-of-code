fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-01 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> i32 {
    input.chars().map(|x| if x == '(' { 1 } else { -1 }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(0, solve_puzzle("(())"));
        assert_eq!(3, solve_puzzle("((("));
    }
}
