fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-04 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let input = input.trim();
    (0..).find(|x| has_zeroes(input, *x)).unwrap()
}

fn has_zeroes(word: &str, number: usize) -> bool {
    let key = format!("{}{}", word, number);
    format!("{:?}", md5::compute(key)).get(..6) == Some("000000")
}
