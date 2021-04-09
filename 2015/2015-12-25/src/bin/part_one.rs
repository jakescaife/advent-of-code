fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-25 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let input: String = input.chars().filter(|&x| x != ',' && x != '.').collect();
    let coordinates: Vec<usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let (row, column) = (coordinates[0], coordinates[1]);
    let code_number = (1..row + column - 1).sum::<usize>() + column;

    (1..code_number).fold(20151125, |code, _| (code * 252533) % 33554393)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        assert_eq!(28094349, solve_puzzle("5 3"));
        assert_eq!(15514188, solve_puzzle("2 5"));
    }
}
