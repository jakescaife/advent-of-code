fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-03 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let lengths: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    lengths.chunks(3).filter(valid_triangle).count()
}

fn valid_triangle(sides: &&[u32]) -> bool {
    let mut triangle = sides.to_vec();
    triangle.sort_unstable();
    triangle[0] + triangle[1] > triangle[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(0, solve_puzzle("5 10 25"));
        assert_eq!(1, solve_puzzle("5 10 10\n5 10 25"));
    }
}
