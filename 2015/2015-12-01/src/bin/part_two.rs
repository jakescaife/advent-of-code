fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-01 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    input
        .chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .scan(0, |s, x| {
            *s += x;
            Some(*s)
        })
        .position(|x| x < 0)
        .map(|x| x + 1)
        .expect("Never entered the basement.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(1, solve_puzzle(")"));
        assert_eq!(5, solve_puzzle("()())"));
    }
}
