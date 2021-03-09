fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-02 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> u32 {
    input.lines().map(|x| calculate_ribbon(x)).sum()
}

fn calculate_ribbon(s: &str) -> u32 {
    let mut dimensions: Vec<u32> = s.split('x').map(|x| x.parse().unwrap()).collect();
    dimensions.sort_unstable();

    dimensions[0] * dimensions[1] * dimensions[2] + 2 * dimensions[0] + 2 * dimensions[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(34, calculate_ribbon("2x3x4"));
        assert_eq!(14, calculate_ribbon("1x1x10"));
    }
}
